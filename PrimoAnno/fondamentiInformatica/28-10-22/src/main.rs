mod binary_tree;
mod utils;

use std::io;

fn main() {
    let mut binary_str = String::new();
    let mut tree_value_input = String::new();

    print!("Enter binary tree structure: ");
    if let Err(msg) = io::stdin().read_line(&mut binary_str) {
        eprintln!("{}", msg);
    }

    if let Err(msg) = utils::check_syntax(&binary_str) {
        eprintln!("{}", msg);
    }

    let mut tree = binary_tree::BinaryTree::new(1);
    let mut num: u8;

    for ch in binary_str.chars() {
	num = 0;
	
        if ch == '1' {
            print!("Enter node value: ");
            io::stdin()
                .read_line(&mut tree_value_input)
                .expect("Couldn't read value from stdin");

            num = tree_value_input.trim().parse().expect("NaN");
        }

	tree.add_node(num, None);
    }
}
