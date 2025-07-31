use std::ops::Index;

pub type NodeId = u32;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub self_count: u64,
    pub total_count: u64,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
}

pub struct FlameTree {
    arena: Vec<Node>,
    parents: Vec<NodeId>,
}

impl Default for FlameTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FlameTree {
    /// Create a new tree with an implicit root node at index 0.
    pub fn new() -> Self {
        let root = Node {
            name: String::from("root"),
            self_count: 0,
            total_count: 0,
            first_child: None,
            next_sibling: None,
        };
        FlameTree {
            arena: vec![root],
            parents: vec![0],
        }
    }

    /// Return the root node id.
    pub fn root(&self) -> NodeId {
        0
    }

    /// Insert a child node under `parent`.
    /// Returns the id of the newly created node.
    pub fn insert_child(&mut self, parent: NodeId, name: &str, self_cnt: u64) -> NodeId {
        let id = self.arena.len() as NodeId;
        let node = Node {
            name: name.to_string(),
            self_count: self_cnt,
            total_count: self_cnt,
            first_child: None,
            next_sibling: self.arena[parent as usize].first_child,
        };
        self.arena[parent as usize].first_child = Some(id);
        self.arena.push(node);
        self.parents.push(parent);
        // Propagate counts up the tree
        let mut p = parent;
        loop {
            self.arena[p as usize].total_count =
                self.arena[p as usize].total_count.saturating_add(self_cnt);
            if p == self.parents[p as usize] {
                break;
            }
            p = self.parents[p as usize];
        }
        id
    }

    /// Total sample count stored at the root node.
    pub fn total_samples(&self) -> u64 {
        self.arena[0].total_count
    }

    /// Mutable access to a node by id.
    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.arena[id as usize]
    }
}

impl Index<NodeId> for FlameTree {
    type Output = Node;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.arena[index as usize]
    }
}
