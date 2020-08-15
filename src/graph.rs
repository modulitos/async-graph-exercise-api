use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

type NodeId = u32;

pub struct Graph {
    map: HashMap<NodeId, Node>,
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Hash, Deserialize)]
pub struct ChildNode(Option<NodeId>);

#[derive(Hash, Deserialize)]
pub struct Node {
    pub left: ChildNode,
    pub right: ChildNode,
    pub score: u32,
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
        // TODO: read graph structure from a json file instead.
        let data = r#"
        [
          {
            "id": 1,
            "score": 2,
            "left": null,
            "right": 3
          },
          {
            "id": 3,
            "score": 4,
            "left": null,
            "right": null
          }
        ]"#;
        let nodes: Vec<Node> = serde_json::from_str(data)?;

        nodes.into_iter().for_each(|node| {
           map.insert(node.id, node);
        });

        Ok(Self { map })
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.map.get(&id)
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
    assert_eq!(node_1.score, 2);
    assert_eq!(node_3.score, 4);
    Ok(())
}
