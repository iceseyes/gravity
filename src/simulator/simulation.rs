use crate::simulator::runner::Runner;
use crate::simulator::{Snapshot, World};
use std::sync::{Arc, RwLock, mpsc};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Simulation {
    runner: Runner,
    snapshot: Snapshot,
}

impl Simulation {
    const FRAME_PER_SECOND: u32 = 60;

    fn new(runner: Runner) -> Self {
        let world = runner.world().clone();
        let snapshot = Arc::new(RwLock::new(world));
        Self { runner, snapshot }
    }

    fn start(&mut self) {
        self.runner.start();
    }

    fn step(&mut self) {
        self.runner.run_once();
        if let Ok(mut snapshot) = self.snapshot.write() {
            snapshot.copy_from(&self.runner.world());
            snapshot.reset_viewport();
        } else {
            println!("Failed to write snapshot");
        }
    }
}

pub fn run(world: World, dt: f32, steps_per_sec: u32) -> (JoinHandle<()>, Snapshot) {
    let mut simulation = Simulation::new(Runner::new(world, dt));
    let snapshot = simulation.snapshot.clone();
    simulation.start();
    let simulation_thread = spawn(move || {
        let steps_per_frame = steps_per_sec / Simulation::FRAME_PER_SECOND; // we support 60 fps
        let frame_duration =
            Duration::from_micros((1e6 / Simulation::FRAME_PER_SECOND as f32) as u64);

        let mut steps = 0u32;
        let mut steps_tot = 0u32;
        let mut t0 = Instant::now();
        let mut t1 = Instant::now();

        loop {
            simulation.step();
            steps_tot += 1;
            steps += 1;

            if steps >= steps_per_frame {
                if let Some(rest) = frame_duration.checked_sub(t0.elapsed()) {
                    println!(
                        "Simulation TOO FAST: waiting for {:.2} ms",
                        rest.as_micros() as f32 / 1e3
                    );
                    sleep(rest);
                }
                steps = 0;
                t0 = Instant::now();
            } else if t0.elapsed() >= frame_duration {
                println!(
                    "Simulation TOO SLOW: {steps} steps in {} ms",
                    t0.elapsed().as_micros() as f32 / 1e3
                );
                t0 = Instant::now();
                steps = 0;
            }

            if t1.elapsed() >= Duration::from_secs(1) {
                println!(
                    "Simulation: {:.2} steps/sec",
                    steps_tot as f32 / (t1.elapsed().as_millis() as f32 / 1000_f32)
                );
                steps_tot = 0;
                t1 = Instant::now();
            }
        }
    });

    (simulation_thread, snapshot)
}
