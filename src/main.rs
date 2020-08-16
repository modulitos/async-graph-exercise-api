#![warn(clippy::all, missing_debug_implementations, rust_2018_idioms)]

mod graph;

use warp::{Filter, Rejection};

use std::{thread, time};
use tokio::task;

use crate::graph::{Graph, Result};
use std::sync::Arc;
use warp::hyper::StatusCode;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let graph = Graph::new()?;
    let graph = Arc::new(graph);

    // GET /node/:u32
    let node = warp::path!("node" / u32)
        .and_then(move |node_id| get_node_info(node_id, Arc::clone(&graph)));

    // We can use the end() filter to match a shorter path
    let help = warp::path("node")
        .and(warp::path::end())
        .map(|| "This is the node API. Try calling /node/:u32");
    let graph = help.or(node);

    let routes = warp::get().and(graph);
    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
    Ok(())
}

async fn get_node_info(node_id: u32, graph: Arc<Graph>) -> Result<impl warp::Reply, Rejection> {
    task::block_in_place(|| {
        // sleep here to mimic a compute-heavy task.
        thread::sleep(time::Duration::from_secs(5));
    });

    if let Some(node) = graph.get(node_id) {
        Ok(warp::reply::json(node))
    } else {
        Err(warp::reject::not_found())
    }
}
