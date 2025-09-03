use std::fmt::Write;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::{self, JoinHandle, sleep},
    time::Duration,
    vec,
};

use env_logger::Target;
use parking_lot::{RwLock, RwLockUpgradableReadGuard, deadlock};
use rand::Rng;

struct Segment {
    thread_seq_nb: usize,
}

impl Segment {
    fn new(thread_seq_nb: usize) -> Self {
        Segment { thread_seq_nb }
    }
}

struct Shard {
    segments: Arc<RwLock<Vec<Segment>>>,
}

impl Shard {
    fn upgrade(&self, thread_seq_nb: usize, op_duration: Duration) {
        log::info!("{thread_seq_nb} requesting upgradable read access");
        let segments_lock = self.segments.upgradable_read();
        log::info!("{thread_seq_nb} got upgradable read access");
        sleep(op_duration);
        log::info!("{thread_seq_nb} requesting upgradable write access");
        let mut write_segments = RwLockUpgradableReadGuard::upgrade(segments_lock);
        write_segments.push(Segment::new(thread_seq_nb));
        log::info!("{thread_seq_nb} got upgradable write access");
        sleep(op_duration);
    }

    fn write(&self, thread_seq_nb: usize, op_duration: Duration) {
        log::info!("{thread_seq_nb} requesting write access");
        let mut write_segments = self.segments.write();
        write_segments.push(Segment::new(thread_seq_nb));
        log::info!("{thread_seq_nb} got write access");
        sleep(op_duration);
    }

    fn read(&self, thread_seq_nb: usize, op_duration: Duration) {
        log::info!("{thread_seq_nb} requesting read access");
        let read_segments = self.segments.read();
        log::info!(
            "{thread_seq_nb} got read access (segments:{})",
            read_segments.len()
        );
        sleep(op_duration);
    }
}

fn main() {
    setup_logger();

    let stopped = Arc::new(AtomicBool::new(false));
    let r = stopped.clone();

    ctrlc::set_handler(move || {
        log::info!("Stress time is stopping");
        r.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    // Number of threads per group
    let readers_count = 5;
    let writers_count = 2;
    let upgraders_count = 5;

    // Sleeping range
    let min_sleep = 20;
    let max_sleep = 200;

    // Ops counter
    let counter = Arc::new(AtomicUsize::new(0));

    // Shared structure
    let segments = Arc::new(RwLock::new(vec![]));
    let shard = Shard { segments };

    // Join handles to stop
    let mut handles = vec![];

    // Setup deadlock detector
    handles.push(deadlock_detector(stopped.clone()));

    // Arc refs
    let shard = Arc::new(shard);
    let shard_clone = shard.clone();
    let stopped_clone = stopped.clone();
    let counter_clone = counter.clone();

    // Some readers
    for i in 0..readers_count {
        let shard = shard_clone.clone();
        let stopped = stopped_clone.clone();
        let counter = counter_clone.clone();

        let t_read_handle = thread::Builder::new()
            .name(format!("read_{i}"))
            .spawn(move || {
                let mut rng = rand::rng();
                while !stopped.load(Ordering::Relaxed) {
                    let duration: u64 = rng.random_range(min_sleep..=max_sleep);
                    let op_duration = Duration::from_millis(duration);
                    let thread_seq_nb = counter.fetch_add(1, Ordering::SeqCst);
                    shard.read(thread_seq_nb, op_duration);
                }
            })
            .unwrap();
        handles.push(t_read_handle);
    }

    // Some writers
    for i in 0..writers_count {
        let shard = shard_clone.clone();
        let stopped = stopped_clone.clone();
        let counter = counter_clone.clone();

        let t_write_handle = thread::Builder::new()
            .name(format!("write_{i}"))
            .spawn(move || {
                let mut rng = rand::rng();
                while !stopped.load(Ordering::Relaxed) {
                    let duration: u64 = rng.random_range(min_sleep..=max_sleep);
                    let op_duration = Duration::from_millis(duration);
                    let thread_seq_nb = counter.fetch_add(1, Ordering::SeqCst);
                    shard.write(thread_seq_nb, op_duration);
                }
            })
            .unwrap();
        handles.push(t_write_handle);
    }

    // Some upgraders
    for i in 0..upgraders_count {
        let shard = shard_clone.clone();
        let stopped = stopped_clone.clone();
        let counter = counter_clone.clone();

        let t_upgrade_handle = thread::Builder::new()
            .name(format!("upgrade_{i}"))
            .spawn(move || {
                let mut rng = rand::rng();
                while !stopped.load(Ordering::Relaxed) {
                    let duration: u64 = rng.random_range(min_sleep..=max_sleep);
                    let op_duration = Duration::from_millis(duration);
                    let thread_seq_nb = counter.fetch_add(1, Ordering::SeqCst);
                    shard.upgrade(thread_seq_nb, op_duration);
                }
            })
            .unwrap();
        handles.push(t_upgrade_handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }
}

pub fn setup_logger() {
    let mut log_builder = env_logger::Builder::new();

    log_builder
        .target(Target::Stdout)
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Info);

    log_builder.init();
}

pub fn deadlock_detector(stopped: Arc<AtomicBool>) -> JoinHandle<()> {
    const DEADLOCK_CHECK_PERIOD: Duration = Duration::from_secs(30);
    thread::Builder::new()
        .name("deadlock_checker".to_string())
        .spawn(move || {
            while !stopped.load(Ordering::Relaxed) {
                thread::sleep(DEADLOCK_CHECK_PERIOD);
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                let mut error = format!("{} deadlocks detected\n", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    writeln!(error, "Deadlock #{i}").expect("fail to writeln!");
                    for t in threads {
                        writeln!(
                            error,
                            "Thread Id {:#?}\n{:#?}",
                            t.thread_id(),
                            t.backtrace(),
                        )
                        .expect("fail to writeln!");
                    }
                }
                log::error!("{error}");
            }
        })
        .unwrap()
}
