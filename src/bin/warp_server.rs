#![warn(clippy::all, missing_debug_implementations, rust_2018_idioms)]

use warp::{Filter, Rejection};

use std::{thread, time};
use tokio::task;

use api::graph::{SerializedNode, Graph, NodeId, Result};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let graph = Graph::new()?;
    let graph = Arc::new(graph);

    // GET /node/:u32
    let node = warp::path!("node" / NodeId)
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

async fn get_node_info(node_id: NodeId, graph: Arc<Graph>) -> Result<impl warp::Reply, Rejection> {
    if let Some(node) = graph.get(node_id) {
        task::block_in_place(|| {
            // sleep here to mimic a compute-heavy task.
            // TODO: sleep the task, instead of the whole thread!
            thread::sleep(time::Duration::from_secs(node.duration));
        });
        Ok(warp::reply::json(&SerializedNode::from(node)))
    } else {
        Err(warp::reject::not_found())
    }
}
