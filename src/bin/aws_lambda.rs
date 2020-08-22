#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate log;
use serde::Deserialize;
extern crate simple_logger;
use once_cell::sync::OnceCell;

use std::{thread, time};
use tokio::task;

use api::graph::{Graph, NodeId, Result, SerializedNode};

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

// Inspired by this implementation: https://github.com/awslabs/aws-lambda-rust-runtime/issues/123

static INSTANCE: OnceCell<Graph> = OnceCell::new();

// The entry point of our bootstrap executable. This is the code that will run when Lambda starts
// our function:

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    // TODO: implement error handling
    INSTANCE.set(Graph::new().unwrap()).ok();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<SerializedNode, HandlerError> {
    let graph = INSTANCE.get().expect("cannot get graph");

    if let Some(node) = graph.get(e.node_id) {
        task::block_in_place(|| {
            // sleep here to mimic a compute-heavy task.
            // TODO: sleep the task, instead of the whole thread!
            thread::sleep(time::Duration::from_secs(node.duration));
        });
        Ok(SerializedNode::from(node))
    } else {
        error!("The node id was not found in the request {}", c.aws_request_id);
        Err(c.new_error(&format!("Node id is not found: {}", e.node_id)))
    }
}
