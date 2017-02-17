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
    pub fn new_with_count(count: usize) -> Self {
        Waiter {
            count: Arc::new(AtomicUsize::new(count)),
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
    #[inline]
    pub fn clone_and_add(&self) -> Self {
        
        self.add(1usize);

        Waiter {
            count: self.count.clone(),
            thread: self.thread.clone(),
        }
    }
}

impl Clone for Waiter {
    #[inline]
    fn clone(&self) -> Self {
        Waiter {
            count: self.count.clone(),
            thread: self.thread.clone(),
        }
    }
}
