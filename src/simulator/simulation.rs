use crate::simulator::runner::Runner;
use crate::simulator::{Snapshot, World};
use std::sync::{Arc, RwLock, mpsc};
use std::thread::{JoinHandle, spawn};
use std::time::Instant;

pub enum SimulationCommand {
    Restart,
    Pause,
    ResetWarning,
    Quit,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum SimulationWarning {
    #[default]
    None,
    SimulationTooSlow,
    SimulationTooFast,
}

#[derive(Debug)]
struct Simulation {
    runner: Runner,
    running: bool,
    desired_steps_per_second: f64,
    last_steps_per_second: f64,

    last_update: Instant,
    accumulator: f64,

    command_rx: mpsc::Receiver<SimulationCommand>,

    snapshot: Snapshot,
}

impl Simulation {
    const MAX_STEPS_PER_FRAME: f64 = 100.0;

    fn new(
        runner: Runner,
        steps_per_second: f64,
        command_rx: mpsc::Receiver<SimulationCommand>,
    ) -> Self {
        let snapshot = Arc::new(RwLock::new(SimulationSnapshot::new(&runner)));
        Self {
            runner,
            running: true,
            desired_steps_per_second: steps_per_second,
            last_steps_per_second: steps_per_second,
            last_update: Instant::now(),
            accumulator: 0.0,
            command_rx,
            snapshot,
        }
    }

    fn start(&mut self) {
        self.running = true;
        self.accumulator = 0.0;
        self.last_update = Instant::now();
        if let Ok(mut snapshot) = self.snapshot.write() {
            snapshot.running = true;
        }
    }

    fn pause(&mut self) {
        self.running = false;
        if let Ok(mut snapshot) = self.snapshot.write() {
            snapshot.running = false;
        }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn step(&mut self) {
        if self.running {
            self.runner.run_once();

            if let Ok(mut snapshot) = self.snapshot.write() {
                snapshot.copy_from(self);
            } else {
                println!("Failed to write snapshot");
            }
        }
    }

    fn update(&mut self) -> bool {
        if !self.running {
            return if !self.receive_command() {
                false
            } else {
                self.last_update = Instant::now();
                true
            };
        }

        let now = Instant::now();
        let elapsed = (now - self.last_update).as_secs_f64();

        self.accumulator += elapsed * self.desired_steps_per_second;

        let steps = self.accumulator.floor().min(Self::MAX_STEPS_PER_FRAME) as usize;
        self.accumulator -= steps as f64;

        self.last_update = now;
        for _ in 0..steps {
            if !self.receive_command() {
                return false;
            }

            self.step();
        }

        self.last_steps_per_second = steps as f64 / self.last_update.elapsed().as_secs_f64();

        if self.last_steps_per_second < self.desired_steps_per_second {
            if let Ok(mut snapshot) = self.snapshot.write() {
                snapshot.warning = SimulationWarning::SimulationTooSlow;
            }
        } else if self.last_steps_per_second > self.desired_steps_per_second
            && let Ok(mut snapshot) = self.snapshot.write()
        {
            snapshot.warning = SimulationWarning::SimulationTooFast;
        }

        true
    }

    fn receive_command(&mut self) -> bool {
        let mut cont = true;
        if let Ok(command) = self.command_rx.try_recv() {
            cont = self.handle(command);
        }

        cont
    }

    fn handle(&mut self, command: SimulationCommand) -> bool {
        match command {
            SimulationCommand::Restart => self.start(),
            SimulationCommand::Pause => self.pause(),
            SimulationCommand::ResetWarning => {
                if let Ok(mut snapshot) = self.snapshot.write() {
                    snapshot.warning = SimulationWarning::None;
                }
            }
            SimulationCommand::Quit => return false,
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct SimulationSnapshot {
    pub world: World,
    pub time: f64,
    pub running: bool,
    pub samples_per_second: f64,
    pub warning: SimulationWarning,
}

impl SimulationSnapshot {
    pub(crate) fn new(runner: &Runner) -> Self {
        Self {
            world: runner.world().clone(),
            time: runner.time(),
            running: true,
            samples_per_second: 0.0,
            warning: SimulationWarning::None,
        }
    }

    fn copy_from(&mut self, simulation: &Simulation) {
        self.world = simulation.runner.world();
        self.time = simulation.runner.time();
        self.running = simulation.is_running();
        self.samples_per_second = simulation.last_steps_per_second;
    }
}

pub fn run(
    world: World,
    dt: f32,
    steps_per_sec: u32,
) -> (JoinHandle<()>, mpsc::Sender<SimulationCommand>, Snapshot) {
    let (tx, rx) = mpsc::channel();
    let mut simulation = Simulation::new(Runner::new(world, dt), steps_per_sec as f64, rx);
    let snapshot = simulation.snapshot.clone();

    simulation.start();

    let simulation_thread = spawn(move || {
        loop {
            if !simulation.update() {
                break;
            }
        }
    });

    (simulation_thread, tx, snapshot)
}
