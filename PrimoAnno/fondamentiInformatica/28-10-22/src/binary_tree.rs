use queues::*;

use crate::node::Node;

pub struct BinaryTree {
    pub root: Box<Node>,
}

impl BinaryTree {
    pub fn new(value: u8, pos: usize) -> Self {
        Self {
            root: Box::new(Node {
                value,
                left: None,
                right: None,
                pos,
            }),
        }
    }

    pub fn add_node(&mut self, value: u8, pos: usize) {
        if value != 0 {
            if (pos as f64) / 2.0 == self.root.pos as f64 {
                self.root.left = Node::new(value, pos); // Add node to the left of root
            } else if ((pos - 1) as f64) / 2.0 == self.root.pos as f64 {
                self.root.right = Node::new(value, pos); // Add node to the right of root
            } else {
                // Recursivly check for the same things as above but for a generic non-root node of the tree.
                // If we don't find a suitable node in the left tree...
                if let Err(_) = BinaryTree::recursive_add_node(&mut self.root.left, value, pos, &1)
                {
                    // ... try looking in the right tree.
                    // If this failes and returns an Err() we can overlook it since it means that the node can't be attached to any tree node.
                    let _ = BinaryTree::recursive_add_node(&mut self.root.right, value, pos, &1);
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        let mut queue: Queue<Node> = Queue::new();
        let mut flag = false;

        queue.add(*self.root.clone()).ok();

        while queue.peek().is_ok() {
            // Get current node - starting at the root of the tree.
            let node = queue.remove().ok().unwrap();
            // If the node has a left branch...
            match node.left {
                Some(node) => {
                    // ... and we found a missing branch before, the tree isn't complete.
                    if flag == true {
                        return false;
                    }
                    // If we didn't then add the node to the queue and continue.
                    queue.add(*node).ok();
                }
                // If the node doesn't have a left branch flag it because it might be not complete depending on the right branch.
                _ => flag = true,
            }
            // If the node has a right branch...
            match node.right {
                Some(node) => {
                    // ... and we found a missing branch before, the tree isn't complete.
                    if flag == true {
                        return false;
                    }
                    // If we didn't then add the node to the queue and continue.
                    queue.add(*node).ok();
                }
                // If the node doesn't have a right branch flag it because it might be not complete depending on the next interation of the loop.
                _ => flag = true,
            }
        }
        // If we complete the loop then the tree is complete.
        true
    }

    pub fn is_full(&self, node: &Option<Box<Node>>) -> bool {
	if node.is_none() {
	    return true;
	}

	let node = node.as_ref().unwrap();

	if node.left.is_none() && node.right.is_none() {
	    return true;
	} 

	if node.left.is_some() && node.right.is_some() {
	    return self.is_full(&node.left) && self.is_full(&node.right);
	}
	
	false
    }

    pub fn is_balanced(&self, node: &Option<Box<Node>>) -> bool {
	if node.is_none() {
	    return true;
	}
	
	let node = node.as_ref().unwrap();
	let left_height: usize = BinaryTree::get_branch_height(&node.left);
	let right_height: usize = BinaryTree::get_branch_height(&node.right);

	if left_height.abs_diff(right_height) <= 1 && self.is_balanced(&node.left) && self.is_balanced(&node.right) {
	    return true;
	}

	false
    }

    pub fn _is_bst(&self) -> bool {
        todo!()
    }

    pub fn print(&self) {
        println!(
            "\n------------------------------------------------\n\
	     Tree view:\
	     \n------------------------------------------------\n\
	     {}\
	     ------------------------------------------------",
            self.traverse_root()
        );
    }

    pub fn traverse_root(&self) -> String {
        let mut str = String::new();
        let mut has_right = false;

        str.push_str("Root value: ");
        str.push_str(&self.root.value.to_string());
        str.push('\n');

        let pointer_right = String::from("    └── Right value: ");
        let pointer_left = match self.root.right {
            Some(_) => {
                has_right = true;
                String::from("    ├── Left value:  ")
            }
            _ => String::from("    └── Left value:  "),
        };

        BinaryTree::traverse_tree(&mut str, &String::from(""), &pointer_left, &self.root.left, has_right);
        BinaryTree::traverse_tree(&mut str, &String::from(""), &pointer_right, &self.root.right, false);

        str
    }
}

impl BinaryTree {
    fn recursive_add_node(current: &mut Option<Box<Node>>, value: u8, pos: usize, prev_pos: &usize) -> Result<(), ()> {
        match current {
            Some(branch) => {
                if (pos as f64) / 2.0 == branch.pos as f64 {
                    let _ = BinaryTree::recursive_add_node(&mut branch.left, value, pos, &branch.pos);
                    return Ok(());
                } else if ((pos - 1) as f64) / 2.0 == branch.pos as f64 {
                    let _ = BinaryTree::recursive_add_node(&mut branch.right, value, pos, &branch.pos);
                    return Ok(());
                } else {
                    if let Err(_) =
                        BinaryTree::recursive_add_node(&mut branch.left, value, pos, &branch.pos)
                    {
                        match BinaryTree::recursive_add_node(&mut branch.right, value, pos, &branch.pos) {
                            Ok(_) => return Ok(()),
                            Err(_) => return Err(()),
                        }
                    }

                    return Ok(());
                }
            }
            _ => {
                if prev_pos * 2 == pos || prev_pos * 2 + 1 == pos {
                    *current = Node::new(value, pos);
                    return Ok(());
                }

                Err(())
            }
        }
    }

    fn traverse_tree(str: &mut String, padding: &String, pointer: &String, node: &Option<Box<Node>>, has_right_branch: bool) {
        match node {
            Some(node) => {
                let mut next_has_right = false;

                str.push_str(&padding);
                str.push_str(&pointer);
                str.push_str(&node.value.to_string());
                str.push('\n');

                let mut padding_builder = String::from(padding);
                match has_right_branch {
                    true => padding_builder.push_str("    │  "),
                    _ => padding_builder.push_str("       "),
                }

                let padding_both = String::from(padding_builder);
                let pointer_right = String::from("    └── Right value: ");
                let pointer_left = match node.right {
                    Some(_) => {
                        next_has_right = true;
                        String::from("    ├── Left value:  ")
                    }
                    _ => String::from("    └── Left value:  "),
                };

                BinaryTree::traverse_tree(str, &padding_both, &pointer_left, &node.left, next_has_right);
                BinaryTree::traverse_tree(str, &padding_both, &pointer_right, &node.right, false);
            }
            _ => {
                str.push_str(&String::from(""));
            }
        }
    }

    fn get_branch_height(node: &Option<Box<Node>>) -> usize {
	if node.is_none() {
	    return 0;
	}

	let node = node.as_ref().unwrap();
	1 + BinaryTree::get_branch_height(&node.left).max(BinaryTree::get_branch_height(&node.right))
    }
}
