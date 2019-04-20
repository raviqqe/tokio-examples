#![feature(async_await, await_macro, futures_api)]

extern crate bdwgc_alloc;
extern crate tokio;
extern crate tokio_async_await;

use tokio::prelude::*;

#[global_allocator]
static ALLOC: bdwgc_alloc::Allocator = bdwgc_alloc::Allocator;

fn main() {
    unsafe { bdwgc_alloc::Allocator::initialize() }

    let mut runtime = tokio::runtime::Builder::new()
        .after_start(move || unsafe {
            println!("Registering a thread ...");
            bdwgc_alloc::Allocator::register_current_thread().unwrap();
        })
        .before_stop(move || unsafe {
            println!("Unregistering a thread ...");
            bdwgc_alloc::Allocator::unregister_current_thread();
        })
        .build()
        .unwrap();

    runtime
        .block_on::<_, _, ()>(tokio_async_await::compat::backward::Compat::new(
            async move {
                let mut buf: [u8; 1024] = [0; 1024];

                let mut stdin = tokio::io::stdin();
                await!(stdin.read_async(&mut buf)).unwrap();

                let mut stdout = tokio::io::stdout();
                await!(stdout.write_async(&mut buf)).unwrap();

                Ok(())
            },
        ))
        .unwrap();
}
