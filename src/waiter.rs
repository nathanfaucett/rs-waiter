use std::thread::{self, Thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};


#[derive(Clone)]
pub struct Waiter {
    count: Arc<AtomicUsize>,
    thread: Thread,
}

impl Waiter {
    pub fn new(count: usize) -> Self {
        Waiter {
            count: Arc::new(AtomicUsize::new(count)),
            thread: thread::current(),
        }
    }

    pub fn done(&self) {
        if self.count.fetch_sub(1usize, Ordering::Relaxed) == 1usize {
            self.thread.unpark();
        }
    }
    pub fn wait(&self) {
        while self.count.load(Ordering::Relaxed) != 0usize {
            thread::park();
        }
    }
}

#[cfg(test)]
mod test {
    extern crate num_cpus;


    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;

    use super::*;


    #[test]
    fn test() {
        let cpus = num_cpus::get();

        let count = 1usize + (cpus * 2usize);
        let counter = Arc::new(AtomicUsize::new(0usize));
        let waiter = Waiter::new(count);

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
}
