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
            count: Arc::new(AtomicUsize::new(1usize)),
            thread: thread::current(),
        }
    }

    #[inline]
    pub fn new_with_count(count: usize) -> Self {
        Waiter {
            count: Arc::new(AtomicUsize::new(count + 1usize)),
            thread: thread::current(),
        }
    }

    #[inline]
    pub fn add(&self, value: usize) -> &Self {
        self.count.fetch_add(value, Ordering::Relaxed);
        self
    }
    #[inline]
    pub fn done(&self) -> Result<(), &'static str> {
        let prev_count = self.count.fetch_sub(1usize, Ordering::Relaxed);

        if prev_count == 1usize {
            Err("done call dropped count below 0")
        } else if prev_count == 2usize {
            self.thread.unpark();
            Ok(())
        } else {
            Ok(())
        }
    }
    #[inline]
    pub fn wait(&self) -> Result<(), &'static str> {
        if self.thread.id() != thread::current().id() {
            Err("can only call wait from thread in which Waiter was created")
        } else {
            while self.count.load(Ordering::Relaxed) != 1usize {
                thread::park();
            }
            Ok(())
        }
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
