waiter [![Build Status](https://travis-ci.org/nathanfaucett/rs-waiter.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-waiter)
=====

waiter for waiting on multiple jobs to finish

```rust
extern crate waiter;


use waiter::Waiter;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;


// known count
fn main() {
    static COUNT: usize = 10;

    let counter = Arc::new(AtomicUsize::new(0));
    let waiter = Waiter::new_with_count(COUNT);

    for _ in 0..COUNT {
        let waiter = waiter.clone();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
            let _ = waiter.done();
        });
    }

    let _ = waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), COUNT);
}
```

```rust
extern crate waiter;


use waiter::Waiter;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;


// unknown count
fn main() {
    static COUNT: usize = 10;

    let counter = Arc::new(AtomicUsize::new(0));
    let waiter = Waiter::new();

    for _ in 0..COUNT {
        let waiter = waiter.clone_and_add();
        let counter = counter.clone();

        let _ = thread::spawn(move || {
            counter.fetch_add(1, Ordering::Relaxed);
            let _ = waiter.done();
        });
    }

    let _ = waiter.wait();

    assert_eq!(counter.load(Ordering::Relaxed), COUNT);
}
```
