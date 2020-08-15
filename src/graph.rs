use std::collections::HashMap;

type NodeId = u32;

pub struct Graph {
    map: HashMap<NodeId, Node>,
}

// TODO: impl Debug for left and right to render custom "null" string
#[derive(Hash)]
pub struct Node {
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub score: u32,
}

impl Graph {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            1,
            Node {
                score: 2,
                left: None,
                right: Some(3),
            },
        );
        map.insert(
            3,
            Node {
                score: 4,
                left: None,
                right: None,
            },
        );
        Self { map }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.map.get(&id)
    }
}
