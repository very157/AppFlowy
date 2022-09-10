use crate::core::{NodeAttributes, TextDelta};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeData {
    pub node_type: String,
    pub attributes: NodeAttributes,
    pub delta: Option<TextDelta>,
}

impl NodeData {
    pub fn new(node_type: &str) -> NodeData {
        NodeData {
            node_type: node_type.into(),
            attributes: NodeAttributes::new(),
            delta: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Node {
    #[serde(rename = "type")]
    pub note_type: String,

    pub attributes: NodeAttributes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<TextDelta>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: &str) -> Node {
        Node {
            note_type: node_type.into(),
            attributes: NodeAttributes::new(),
            delta: None,
            children: Vec::new(),
        }
    }

    pub fn to_node_data(&self) -> NodeData {
        NodeData {
            node_type: self.note_type.clone(),
            attributes: self.attributes.clone(),
            delta: self.delta.clone(),
        }
    }
}
