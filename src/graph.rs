use std::collections::HashMap;
use std::fmt::Debug;

type NodeId = u32;

pub struct Graph {
    map: HashMap<NodeId, Node>,
}

#[derive(Hash)]
pub struct ChildNode(Option<NodeId>);

#[derive(Hash)]
pub struct Node {
    pub left: ChildNode,
    pub right: ChildNode,
    pub score: u32,
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
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // TODO: read graph structure from a json file instead.
        map.insert(
            1,
            Node {
                score: 2,
                left: ChildNode(None),
                right: ChildNode(Some(3)),
            },
        );
        map.insert(
            3,
            Node {
                score: 4,
                left: ChildNode(None),
                right: ChildNode(None),
            },
        );
        Self { map }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.map.get(&id)
    }
}
