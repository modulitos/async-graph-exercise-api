#![warn(clippy::all, missing_debug_implementations, rust_2018_idioms)]

use warp::Filter;

use std::{thread, time};
use tokio::task;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /node/:u32
    let node = warp::path!("node" / u32).map(|node_id| {
        task::block_in_place(|| {
            // sleep here to mimic a compute-heavy task.
            thread::sleep(time::Duration::from_secs(5));
        });
        format!("visiting node id: {}", node_id)
    });
    // GET /quick_node/:u32
    // Enables e2e testing to ensure we are processing requests concurrently.
    let quick_node =
        warp::path!("quick_node" / u32).map(|node_id| format!("visiting node id: {}", node_id));
    let node = node.or(quick_node);

    // We can use the end() filter to match a shorter path
    let help = warp::path("node")
        .and(warp::path::end())
        .map(|| "This is the node API. Try calling /node/:u32");
    let graph = help.or(node);

    let routes = warp::get().and(graph);
    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}
