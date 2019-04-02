#![feature(async_await, await_macro, futures_api)]

extern crate tokio;

use tokio::prelude::*;

fn main() {
    future_in_thread_in_future();
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
