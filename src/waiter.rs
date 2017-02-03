use std::convert::From;
use std::thread::{self, Thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};


pub struct Waiter {
    count: Arc<AtomicUsize>,
    thread: Thread,
}

impl Waiter {
    #[inline]
    pub fn new() -> Self {
        Waiter {
            count: Arc::new(AtomicUsize::new(0usize)),
            thread: thread::current(),
        }
    }

    #[inline]
    pub fn add(&self, value: usize) -> &Self {
        self.count.fetch_add(value, Ordering::Relaxed);
        self
    }
    #[inline]
    pub fn done(&self) -> &Self {
        if self.count.fetch_sub(1usize, Ordering::Relaxed) == 1usize {
            self.thread.unpark();
        }
        self
    }
    #[inline]
    pub fn wait(&self) -> &Self {
        while self.count.load(Ordering::Relaxed) != 0usize {
            thread::park();
        }
        self
    }
}

impl From<usize> for Waiter {
    #[inline]
    fn from(count: usize) -> Self {
        Waiter {
            count: Arc::new(AtomicUsize::new(count)),
            thread: thread::current(),
        }
    }
}

impl Clone for Waiter {
    #[inline]
    fn clone(&self) -> Self {
        self.add(1usize);

        Waiter {
            count: self.count.clone(),
            thread: self.thread.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    extern crate num_cpus;


    use std::thread;
    use std::sync::{Arc, Barrier};
    use std::sync::atomic::AtomicUsize;

    use self::test::Bencher;

    use super::*;


    #[test]
    fn test_waiter() {
        let cpus = num_cpus::get();
        let count = 1usize + (cpus * 2usize);

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
    }

    #[bench]
    fn bench_waiter(b: &mut Bencher) {
        let cpus = num_cpus::get();
        let count = 1usize + (cpus * 2usize);

        b.iter(|| {
            let counter = Arc::new(AtomicUsize::new(0usize));
            let waiter = Waiter::new();

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
}
