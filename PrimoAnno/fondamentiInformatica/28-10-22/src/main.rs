mod node;
mod binary_tree;
mod utils;

use std::{io, process::ExitCode};

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
    println!("Enter root node value: ");
    io::stdin()
        .read_line(&mut tree_value_input)
        .expect("Couldn't read value from stdin");
    let mut tree_root =
        binary_tree::BinaryTree::new(tree_value_input.trim().parse().expect("NaN"), 1);

    for (pos, ch) in binary_str.chars().enumerate() {
        let mut tree_value_input = String::new();
        num = 0;

        if ch == '1' {
	    num = loop {
		println!("Enter node value: ");
		io::stdin()
                    .read_line(&mut tree_value_input)
                    .expect("Couldn't read value from stdin");
		
		let num: u8 = tree_value_input.trim().parse().expect("NaN");
		if num > 0 {
		    break num;
		} else {
		    eprintln!("Invalid value: value should be greater then 0");
		}
	    };
        }

        tree_root.add_node(num, pos+2);
    }

    tree_root.print();

    ExitCode::SUCCESS
}
