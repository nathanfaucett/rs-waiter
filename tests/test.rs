extern crate num_cpus;

extern crate waiter;


use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use waiter::Waiter;


#[test]
fn test_waiter_known_count() {
    let cpus = num_cpus::get();
    let count = 1 + (cpus * 2);

    let counter = Arc::new(AtomicUsize::new(0));
    let waiter = Waiter::new_with_count(count);

    for _ in 0..count {
        let waiter = waiter.clone();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
            let _ = waiter.done();
        });
    }

    let _ = waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), count);
}

#[test]
fn test_waiter_unknown_count() {
    let cpus = num_cpus::get();
    let count = 1 + (cpus * 2);

    let counter = Arc::new(AtomicUsize::new(0));
    let waiter = Waiter::new();

    for _ in 0..count {
        let waiter = waiter.clone_and_add();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
            let _ = waiter.done();
        });
    }

    let _ = waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), count);
}

#[test]
fn test_waiter_panic_if_wait_called_on_wrong_thread() {
    let waiter = Waiter::new_with_count(1);

    let handle = thread::spawn(move || {
        let _ = waiter.done();
        waiter.wait()
    });

    assert_eq!(
        handle.join().unwrap(),
        Err("can only call wait from thread in which Waiter was created")
    );
}

#[test]
fn test_waiter_panic_if_done_called_to_many_times() {
    let waiter = Waiter::new_with_count(1);

    let handle0 = {
        let waiter = waiter.clone();
        thread::spawn(move || { waiter.done() })
    };
    let handle1 = {
        let waiter = waiter.clone();
        thread::spawn(move || { waiter.done() })
    };

    let mut panicked = None;

    match handle0.join() {
        Ok(s) => { panicked = Some(s); },
        Err(_) => (),
    }
    match handle1.join() {
        Ok(s) => { panicked = Some(s); },
        Err(_) => (),
    }

    assert_eq!(panicked, Some(Err("done call dropped count below 0")));
}
