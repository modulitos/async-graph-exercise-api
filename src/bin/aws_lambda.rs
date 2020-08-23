#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate log;
use serde::Deserialize;
extern crate simple_logger;
use once_cell::sync::OnceCell;

use std::{thread, time};
use tokio::task;

use api::error::Result;
use api::graph::{Graph, NodeId, SerializedNode};

use lambda::error::HandlerError;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "nodeId")]
    node_id: NodeId,
}

// Inspired by this implementation: https://github.com/awslabs/aws-lambda-rust-runtime/issues/123

static INSTANCE: OnceCell<Graph> = OnceCell::new();

// The entry point of our bootstrap executable. This is the code that will run when Lambda starts
// our function:

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;
    INSTANCE.set(Graph::new()?).ok();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<SerializedNode, HandlerError> {
    let graph = INSTANCE
        .get()
        .ok_or_else(|| c.new_error("unable to retrieve graph instance"))?;

    if let Some(node) = graph.get(e.node_id) {
        if node.duration > 0 {
            // sleep here to mimic a compute-heavy task.
            task::block_in_place(|| {
                // TODO: sleep the task, instead of the whole thread!
                thread::sleep(time::Duration::from_secs(node.duration));
            });
        }
        Ok(SerializedNode::from(node))
    } else {
        error!(
            "The node id was not found in the request {}",
            c.aws_request_id
        );
        Err(c.new_error(&format!("Node id is not found: {}", e.node_id)))
    }
}
