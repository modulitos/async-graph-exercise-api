use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

type NodeId = u32;

pub struct Graph {
    map: HashMap<NodeId, Node>,
}

type Error = Box<dyn std::error::Error>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Hash, Deserialize, Serialize)]
pub struct ChildNode(Option<NodeId>);

#[derive(Hash, Deserialize, Serialize)]
pub struct Node {
    pub left: ChildNode,
    pub right: ChildNode,
    pub score: u32,
    // the duration of time it takes to process this node:
    pub duration: u64,
    id: NodeId,
}

impl Debug for ChildNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = self.0 {
            f.write_fmt(format_args!("{}", n))
        } else {
            f.write_str("null")
        }
    }
}

impl Graph {
    pub fn new() -> Result<Self> {
        let mut map = HashMap::new();

        // Instantiate the graph from our `graph.json` file by deserializing it from the JSON data.

        let data = std::fs::read_to_string("./graph.json")?.parse::<String>()?;
        let nodes: Vec<Node> = serde_json::from_str(&data)?;

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
                total += next_node.score;
                if let Some(left_id) = next_node.left.0 {
                    nodes_to_visit.push_back(left_id)
                }
                if let Some(right_id) = next_node.right.0 {
                    nodes_to_visit.push_back(right_id)
                }
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
    let node_1 = graph.get(1).unwrap();
    let node_3 = graph.get(3).unwrap();
    assert_eq!(node_1.score, 100);
    assert_eq!(node_3.score, 0);
    Ok(())
}

#[test]
fn graph_get_non_existent() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get(4).is_none(), true);
    Ok(())
}

#[test]
fn graph_sum() -> Result<()> {
    let graph = Graph::new()?;
    assert_eq!(graph.get_total(1), 1750);
    Ok(())
}
