#[allow(dead_code)]

use std::future::Future;

fn main() {
    println!("Hello, world!");
}

async fn foo() -> usize { 0 }

fn bar() -> impl Future<Output = usize> { async { 0 } }

// foo and bar are the same
