use serde::{ Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

pub type NodeId = char;

pub struct Graph {
    map: HashMap<NodeId, Node>,
}

use crate::error::{Result};

#[derive(Hash, Deserialize)]
pub struct Node {
    pub children: Vec<NodeId>,
    pub reward: u32,
    // the duration of time it takes to process this node:
    pub duration: u64,
    id: NodeId,
}

/// Represents the serialized representation of a graph's node.
#[derive(Serialize)]
pub struct SerializedNode {
    // TODO: consider mapping this into a string of the full URL
    children: Vec<NodeId>,
    reward: u32,
}

impl From<&Node> for SerializedNode {
    fn from(node: &Node) -> Self {
        SerializedNode {
            reward: node.reward,
            children: node.children.clone(),
        }
    }
}

impl Graph {
    pub fn new() -> Result<Self> {
        let mut map = HashMap::new();

        // Instantiate the graph from our `graph.json` file by deserializing it from the JSON data.
        // Using the include_str macro to read the JSON into a str at compile time.

        let data = std::include_str!("graph.json");
        let nodes: Vec<Node> = serde_json::from_str(data)?;

        nodes.into_iter().for_each(|node| {
            map.insert(node.id, node);
        });

        Ok(Self { map })
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.map.get(&id)
    }

    // this method helps with testing.
    fn get_total(&self, id: NodeId) -> u32 {
        let mut nodes_to_visit = VecDeque::new();
        nodes_to_visit.push_back(id);
        let mut total = 0;
        loop {
            if let Some(next_node_id) = nodes_to_visit.pop_front() {
                let next_node = self
                    .map
                    .get(&next_node_id)
                    .expect(&format!("non-existent node id: {}", next_node_id));
                total += next_node.reward;
                nodes_to_visit.append(&mut VecDeque::from_iter(next_node.children.iter().cloned()))
            } else {
                // all nodes have been visited
                return total;
            }
        }
    }
}

#[test]
fn graph_new() -> Result<()> {
    Graph::new()?;
    Ok(())
}

#[test]
fn graph_get() -> Result<()> {
    let graph = Graph::new()?;
    let node_1 = graph.get('a').unwrap();
    let node_3 = graph.get('c').unwrap();
    assert_eq!(node_1.reward, 100);
    assert_eq!(node_3.reward, 0);
    Ok(())
}

#[test]
fn graph_get_non_existent() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get('d').is_none(), true);
    Ok(())
}

#[test]
fn graph_sum_e() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get_total('e'), 2150);
    Ok(())
}

#[test]
fn graph_sum_c() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get_total('c'), 1600);
    Ok(())
}

#[test]
fn graph_sum() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get_total('a'), 4250);
    Ok(())
}
