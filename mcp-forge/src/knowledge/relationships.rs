use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub strength: f64,
}

pub struct RelationshipMapper {
    graph: Graph<String, String>,
    relationships: Vec<Relationship>,
    node_indices: HashMap<String, NodeIndex>,
}

impl RelationshipMapper {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            relationships: Vec::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: String) {
        let index = self.graph.add_node(symbol.clone());
        self.node_indices.insert(symbol, index);
    }

    pub fn add_relationship(&mut self, from: String, to: String, kind: String, strength: f64) {
        if let (Some(&from_index), Some(&to_index)) = (self.node_indices.get(&from), self.node_indices.get(&to)) {
            self.graph.add_edge(from_index, to_index, kind.clone());
            self.relationships.push(Relationship { from, to, kind, strength });
        }
    }

    pub fn get_relationships(&self) -> &Vec<Relationship> {
        &self.relationships
    }
}