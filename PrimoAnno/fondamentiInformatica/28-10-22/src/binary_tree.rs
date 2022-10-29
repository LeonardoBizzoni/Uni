use crate::node::Node;

pub struct BinaryTree {
    pub root: Box<Node>,
}

impl BinaryTree {
    pub fn new(value: u8, pos: usize) -> Self {
        Self { root: Box::new(Node { value, left: None, right: None, pos }) }
    }

    pub fn print(self) -> String {
	let mut res = String::new();
	BinaryTree::traverse_tree(&mut res, &String::from(""), &String::from(""), &Some(self.root));

	res
    }

    fn traverse_tree(str: &mut String, padding: &String, pointer: &String, node: &Option<Box<Node>>) {
	if let Some(node) = node {
	    str.push_str(&padding);
	    str.push_str(&pointer);
	    str.push_str("Node value: ");
	    str.push_str(&node.value.to_string());
	    str.push('\n');

	    let mut padding_builder = String::from(padding);
	    padding_builder.push_str("│  ");

	    let padding_both = String::from(padding_builder);
	    let pointer_right = String::from("└──");
	    let pointer_left = match node.right {
		Some(_) => String::from("├──"),
		None => String::from("└──")
	    };
	    
	    BinaryTree::traverse_tree(str, &padding_both, &pointer_left, &node.left);
	    BinaryTree::traverse_tree(str, &padding_both, &pointer_right, &node.right);
	}
    }

    fn recursive_add_node(current: &mut Option<Box<Node>>, value: u8, pos: usize, prev_pos: &usize) -> Result<(), ()>{
	match current {
	    Some(branch) => {
		if (pos as f64)/2.0 == branch.pos as f64 {
		    // add left
		    let _ = BinaryTree::recursive_add_node(&mut branch.left, value, pos, &branch.pos);
		    return Ok(());
		} else if ((pos-1) as f64)/2.0 == branch.pos as f64 {
		    // add right
		    let _ = BinaryTree::recursive_add_node(&mut branch.right, value, pos, &branch.pos);
		    return Ok(());
		} else {
		    if let Err(_) = BinaryTree::recursive_add_node(&mut branch.left, value, pos, &branch.pos) {
			match BinaryTree::recursive_add_node(&mut branch.right, value, pos, &branch.pos) {
			    Ok(_) => return Ok(()),
			    Err(_) => return Err(())
			}
		    }

		    return Ok(());
		}
	    }
	    None => {
		if prev_pos*2 == pos || prev_pos * 2 +1 == pos {
		    *current = Some(Box::new(Node {value, left: None, right: None, pos}));
		    return Ok(());
		}

		Err(())
	    }
	}
    }

    pub fn add_node(&mut self, value: u8, pos: usize) {
        if value != 0 {
	    if (pos as f64)/2.0 == self.root.pos as f64 {
		// add left
		self.root.left = Some(Box::new(Node { value, left: None, right: None, pos }));
	    } else if ((pos-1) as f64)/2.0 == self.root.pos as f64 {
		// add right
		self.root.right = Some(Box::new(Node { value, left: None, right: None, pos }));
	    } else {
		if let Err(_) = BinaryTree::recursive_add_node(&mut self.root.left, value, pos, &1) {
		    let _ = BinaryTree::recursive_add_node(&mut self.root.right, value, pos, &1);
		}
	    }
        }
    }
}
