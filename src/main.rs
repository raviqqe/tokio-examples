#![feature(async_await, await_macro, futures_api)]

extern crate coroutine;
#[macro_use]
extern crate tokio;

use coroutine::asymmetric::Coroutine;
use tokio::prelude::*;

fn main() {
    awaited_future();
}

// success
#[allow(unused)]
fn future_in_thread() {
    let handle = std::thread::spawn(|| {
        tokio::run(future::lazy(move || {
            tokio::spawn_async(async move {});
            println!("Hello, world!");
            Ok(())
        }));
    });

    handle.join().unwrap();
}

// panic
#[allow(unused)]
fn future_in_thread_in_future() {
    tokio::run(future::lazy(move || {
        let handle = std::thread::spawn(|| {
            tokio::spawn_async(async move {});
            println!("Hello, world!");
        });

        handle.join().unwrap();
        Ok(())
    }));
}

// success
#[allow(unused)]
fn print_in_spawn_async() {
    let handle = std::thread::spawn(|| {
        tokio::run_async(async move {
            tokio::spawn_async(async move {
                println!("Hello, coroutine!");
            });
        });
    });

    handle.join().unwrap();
}

// success
#[allow(unused)]
fn coroutine() {
    let handle = std::thread::spawn(|| {
        tokio::run_async(async move {
            let mut handle = Coroutine::spawn(|c, _| {
                tokio::spawn_async(async move {
                    println!("Hello, coroutine!");
                });
                42
            });
            println!("{:?}", handle.resume(0));
        });
    });

    handle.join().unwrap();
}

// illegal hardware instruction
#[allow(unused)]
fn undone_coroutine() {
    let mut handle = Coroutine::spawn(|c, _| {
        c.yield_with(0);
        42
    });

    println!("{:?}", handle.resume(0));
}

// success
#[allow(unused)]
fn parked_coroutine() {
    let handle = std::thread::spawn(|| {
        tokio::run_async(async move {
            let mut handle = Coroutine::spawn(|c, _| {
                tokio::spawn_async(async move {
                    println!("Hello, coroutine!");
                });
                c.park_with(0);
                42
            });

            println!("{:?}", handle.resume(0));
            println!("{:?}", handle.resume(0));
        });
    });

    handle.join().unwrap();
}

// stall
#[allow(unused)]
fn awaited_future_in_coroutine() {
    let handle = std::thread::spawn(|| {
        tokio::run_async(async move {
            let mut handle = Coroutine::spawn(|c, _| {
                let mutex = std::sync::Arc::new(std::sync::Mutex::new(std::cell::Cell::new(false)));
                let cloned = mutex.clone();

                tokio::spawn_async(async move {
                    println!("Hello, coroutine!");
                    *cloned.lock().unwrap().get_mut() = true;
                });
                c.park_with(0);

                while (!mutex.lock().unwrap().get()) {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    println!("waiting...");
                }

                42
            });

            println!("{:?}", handle.resume(0));
            println!("{:?}", handle.resume(0));
        });
    });

    handle.join().unwrap();
}

// success
#[allow(unused)]
fn awaited_future() {
    tokio::run_async(async move {
        let mutex = std::sync::Arc::new(std::sync::Mutex::new(std::cell::Cell::new(false)));
        let cloned = mutex.clone();

        tokio::spawn_async(async move {
            println!("Hello, coroutine!");
            *cloned.lock().unwrap().get_mut() = true;
        });

        await!(tokio::timer::Delay::new(
            std::time::Instant::now() + std::time::Duration::from_millis(100),
        ));

        while (!mutex.lock().unwrap().get()) {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("waiting...");
        }
    });
}
