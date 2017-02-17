extern crate num_cpus;

extern crate waiter;


use std::thread;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};

use waiter::Waiter;


#[test]
fn test_waiter_known_count() {
    let cpus = num_cpus::get();
    let count = 1usize + (cpus * 2usize);

    let counter = Arc::new(AtomicUsize::new(0usize));
    let waiter = Waiter::new_with_count(count);

    for _ in 0usize..count {
        let waiter = waiter.clone();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1usize, Ordering::Relaxed);
            waiter.done();
        });
    }

    waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), count);
}
#[test]
fn test_waiter_unknown_count() {
    let cpus = num_cpus::get();
    let count = 1usize + (cpus * 2usize);

    let counter = Arc::new(AtomicUsize::new(0usize));
    let waiter = Waiter::new();

    for _ in 0usize..count {
        let waiter = waiter.clone_and_add();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1usize, Ordering::Relaxed);
            waiter.done();
        });
    }

    waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), count);
}
