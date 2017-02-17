#![feature(test)]


extern crate test;
extern crate num_cpus;

extern crate waiter;


use std::thread;
use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};

use test::Bencher;

use waiter::Waiter;


#[bench]
fn bench_waiter(b: &mut Bencher) {
    let cpus = num_cpus::get();
    let count = 1usize + (cpus * 2usize);

    b.iter(|| {
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
    });
}
#[bench]
fn bench_barrier(b: &mut Bencher) {
    let cpus = num_cpus::get();
    let count = 1usize + (cpus * 2usize);

    b.iter(|| {
        let counter = Arc::new(AtomicUsize::new(0usize));
        let barrier = Arc::new(Barrier::new(count + 1));

        for _ in 0usize..count {
            let barrier = barrier.clone();
            let counter = counter.clone();

            let _ = thread::spawn(move || {
                counter.fetch_add(1usize, Ordering::Relaxed);
                barrier.wait();
            });
        }

        barrier.wait();

        assert_eq!(counter.load(Ordering::Relaxed), count);
    });
}
