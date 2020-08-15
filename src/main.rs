#![warn(
clippy::all,
missing_debug_implementations,
rust_2018_idioms,
)]
#![deny(warnings)]

use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| {
        println!("processing request!");
        "Hello, World!"
    });

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
