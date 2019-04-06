#![feature(async_await, await_macro, futures_api)]

extern crate coroutine;
extern crate tokio;

use coroutine::asymmetric::Coroutine;
use tokio::prelude::*;

fn main() {
    coroutine_with_multi_therads();
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

// success
#[allow(unused)]
fn coroutine_with_multi_therads() {
    let handle1 = std::thread::spawn(|| {
        tokio::run_async(async move { std::thread::sleep(std::time::Duration::from_millis(3000)) });
    });

    let handle2 = std::thread::spawn(|| {
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

    handle2.join().unwrap();
    handle1.join().unwrap();
}
