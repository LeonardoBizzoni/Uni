pub struct Node {
    pub value: u8,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    pub visited: bool,
}

pub struct BinaryTree {
    pub root: Node,
}

impl BinaryTree {
    pub fn new(value: u8) -> Self {
        Self {
            root: Node {
                value,
                left: None,
                right: None,
                visited: true,
            },
        }
    }

    pub fn add_node(&mut self, value: u8, current_node: Option<Box<Node>>) {
        // Spaghetti code incoming
        if let Some(mut current_node) = current_node {
            // current_node must be a generic non-root Node

            if current_node.left.is_none() {
                current_node.left = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                    visited: false,
                }));
            } else if current_node.right.is_none() {
                current_node.right = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                    visited: false,
                }));
            } else {
                self.add_node(value, Some(current_node.left.unwrap()));
            }
        } else {
            // current_node must be root

            if self.root.left.is_none() {
                self.root.left = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                    visited: false,
                }));
            } else if self.root.right.is_none() {
                self.root.right = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                    visited: false,
                }));
            } else {
                // if root-left !visited call add_node(root-left)
                // else call add_node(root-right)
            }
        }
    }
}
