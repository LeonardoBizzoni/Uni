mod node;
mod binary_tree;
mod utils;

use std::{io::{self, Write}, process::ExitCode};

fn main() -> ExitCode {
    let mut binary_str = String::new();
    let mut tree_value_input = String::new();
    let mut num: u8;

    println!("Enter binary tree structure: ");
    if let Err(msg) = io::stdin().read_line(&mut binary_str) {
        eprintln!("{}", msg);
        return ExitCode::FAILURE;
    }
    let mut binary_str = binary_str.trim().to_string();

    if let Err(msg) = utils::check_syntax(&binary_str) {
        eprintln!("{}", msg);
        return ExitCode::FAILURE;
    }

    binary_str.remove(0);
    print!("Enter root node value: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut tree_value_input)
        .expect("Couldn't read value from stdin");
    let mut tree_root =
        binary_tree::BinaryTree::new(tree_value_input.trim().parse().expect("NaN"), 1);

    for (pos, ch) in binary_str.chars().enumerate() {
        let mut tree_value_input = String::new();

        if ch == '1' {
	    print!("Enter node value: ");
	    let _ = io::stdout().flush();
	    io::stdin()
                .read_line(&mut tree_value_input)
                .expect("Couldn't read value from stdin");
	    num = tree_value_input.trim().parse().expect("NaN");

            tree_root.add_node(num, pos+2);
        }
    }

    tree_root.print();
    println!("Complete: {}", tree_root.is_complete());
    println!("Full: {}", tree_root.is_full(&Some(tree_root.root.clone())));
    println!("Balanced: {}", tree_root.is_balanced(&Some(tree_root.root.clone())));
    // println!("BST: {}", tree_root.is_bst());

    ExitCode::SUCCESS
}
