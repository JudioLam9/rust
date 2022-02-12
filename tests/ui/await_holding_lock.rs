#![warn(clippy::await_holding_lock)]

use std::sync::{Mutex, RwLock};

async fn bad(x: &Mutex<u32>) -> u32 {
    let guard = x.lock().unwrap();
    baz().await
}

async fn good(x: &Mutex<u32>) -> u32 {
    {
        let guard = x.lock().unwrap();
        let y = *guard + 1;
    }
    baz().await;
    let guard = x.lock().unwrap();
    47
}

pub async fn bad_rw(x: &RwLock<u32>) -> u32 {
    let guard = x.read().unwrap();
    baz().await
}

pub async fn bad_rw_write(x: &RwLock<u32>) -> u32 {
    let mut guard = x.write().unwrap();
    baz().await
}

pub async fn good_rw(x: &RwLock<u32>) -> u32 {
    {
        let guard = x.read().unwrap();
        let y = *guard + 1;
    }
    {
        let mut guard = x.write().unwrap();
        *guard += 1;
    }
    baz().await;
    let guard = x.read().unwrap();
    47
}

async fn baz() -> u32 {
    42
}

async fn also_bad(x: &Mutex<u32>) -> u32 {
    let first = baz().await;

    let guard = x.lock().unwrap();

    let second = baz().await;

    let third = baz().await;

    first + second + third
}

async fn not_good(x: &Mutex<u32>) -> u32 {
    let first = baz().await;

    let second = {
        let guard = x.lock().unwrap();
        baz().await
    };

    let third = baz().await;

    first + second + third
}

#[allow(clippy::manual_async_fn)]
fn block_bad(x: &Mutex<u32>) -> impl std::future::Future<Output = u32> + '_ {
    async move {
        let guard = x.lock().unwrap();
        baz().await
    }
}

fn main() {
    let m = Mutex::new(100);
    good(&m);
    bad(&m);
    also_bad(&m);
    not_good(&m);
    block_bad(&m);
}
