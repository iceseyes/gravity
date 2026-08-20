use crate::simulator::runner::Runner;
use crate::simulator::{Snapshot, World};
use std::sync::{Arc, RwLock, mpsc};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

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
    snapshot: Snapshot,
}

impl Simulation {
    const FRAME_PER_SECOND: u32 = 60;

    fn new(runner: Runner) -> Self {
        let snapshot = Arc::new(RwLock::new(SimulationSnapshot::new(&runner)));
        Self { runner, snapshot }
    }

    fn start(&mut self) {
        self.runner.start();
    }

    fn step(&mut self) {
        self.runner.run_once();

        if let Ok(mut snapshot) = self.snapshot.write() {
            snapshot.copy_from(&self.runner);
        } else {
            println!("Failed to write snapshot");
        }
    }

    fn handle(&mut self, command: SimulationCommand) -> bool {
        match command {
            SimulationCommand::Restart => self.start(),
            SimulationCommand::Pause => self.runner.stop(),
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
    pub samples_per_second: f32,
    pub warning: SimulationWarning,
}

impl SimulationSnapshot {
    pub(crate) fn new(runner: &Runner) -> Self {
        Self {
            world: runner.world().clone(),
            time: runner.time(),
            running: runner.is_running(),
            samples_per_second: 0.0,
            warning: SimulationWarning::None,
        }
    }

    fn copy_from(&mut self, runner: &Runner) {
        self.world = runner.world();
        self.time = runner.time();
        self.running = runner.is_running();
    }
}

pub fn run(
    world: World,
    dt: f32,
    steps_per_sec: u32,
) -> (JoinHandle<()>, mpsc::Sender<SimulationCommand>, Snapshot) {
    let (tx, rx) = mpsc::channel();
    let mut simulation = Simulation::new(Runner::new(world, dt));
    let snapshot = simulation.snapshot.clone();

    simulation.start();

    let simulation_thread = spawn(move || {
        let snapshot = simulation.snapshot.clone();
        let steps_per_frame = steps_per_sec / Simulation::FRAME_PER_SECOND;
        let frame_duration =
            Duration::from_micros((1e6 / Simulation::FRAME_PER_SECOND as f32) as u64);

        let mut frame_steps = 0u32;
        let mut steps_per_second_count = 0u32;
        let mut frame_timer = Instant::now();
        let mut fps_timer = Instant::now();

        loop {
            if let Ok(command) = rx.try_recv() {
                let cont = simulation.handle(command);

                if !cont {
                    break;
                }
            }

            simulation.step();
            steps_per_second_count += 1;
            frame_steps += 1;

            if frame_steps >= steps_per_frame {
                if let Some(rest) = frame_duration.checked_sub(frame_timer.elapsed()) {
                    if let Ok(mut snapshot) = snapshot.write() {
                        snapshot.warning = SimulationWarning::SimulationTooFast;
                    }

                    sleep(rest);
                }

                frame_steps = 0;
                frame_timer = Instant::now();
            } else if frame_timer.elapsed() >= frame_duration {
                if let Ok(mut snapshot) = snapshot.write() {
                    snapshot.warning = SimulationWarning::SimulationTooSlow;
                }

                frame_timer = Instant::now();
                frame_steps = 0;
            }

            if fps_timer.elapsed() >= Duration::from_secs(1) {
                if let Ok(mut snapshot) = snapshot.write() {
                    snapshot.samples_per_second =
                        steps_per_second_count as f32 / fps_timer.elapsed().as_secs_f32();
                }

                steps_per_second_count = 0;
                fps_timer = Instant::now();
            }
        }
    });

    (simulation_thread, tx, snapshot)
}
