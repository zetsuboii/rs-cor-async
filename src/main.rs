#[allow(dead_code)]

use std::future::Future;

fn main() {
    println!("Hello, world!");
}

async fn foo() {}

fn bar() -> Future<Output = ()> { async {} }

// foo and bar are the same
