#![warn(clippy::all, missing_debug_implementations, rust_2018_idioms)]

mod graph;

use warp::Filter;

use std::{thread, time};
use tokio::task;

use crate::graph::Graph;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // TODO: add proper error handling here
    let graph = Graph::new().unwrap();
    let graph = Arc::new(graph);

    // GET /node/:u32
    let node =
        warp::path!("node" / u32).map(move |node_id| get_node_info(node_id, Arc::clone(&graph)));

    // We can use the end() filter to match a shorter path
    let help = warp::path("node")
        .and(warp::path::end())
        .map(|| "This is the node API. Try calling /node/:u32");
    let graph = help.or(node);

    let routes = warp::get().and(graph);
    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

fn get_node_info(node_id: u32, graph: Arc<Graph>) -> String {
    task::block_in_place(|| {
        // sleep here to mimic a compute-heavy task.
        thread::sleep(time::Duration::from_secs(5));
    });

    if let Some(node) = graph.get(node_id) {
        format!(
            "visiting node id: {}\n  score: {}\n  left child: {:?}\n  right child: {:?}\n",
            node_id, node.score, node.left, node.right
        )
    } else {
        // TODO: this should return a 404 instead.
        format!("no node found for node id: {}", node_id)
    }
}
