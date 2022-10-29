mod binary_tree;
mod utils;

use std::{io, process::ExitCode};

fn main() -> ExitCode {
    let mut binary_str = String::new();
    let mut tree_value_input = String::new();
    // let mut num: u8;

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
    let mut tree_root = binary_tree::BinaryTree::new(tree_value_input.trim().parse().expect("NaN"), 1);

    tree_root.add_node(2, 2);
    tree_root.add_node(30, 3);

    tree_root.add_node(4, 4);
    tree_root.add_node(6, 6);

    tree_root.add_node(8, 8);
    tree_root.add_node(13, 13);

    tree_root.add_node(15, 15);
    tree_root.add_node(17, 17);

    // for (pos, ch) in binary_str.chars().enumerate() {
    //     let mut tree_value_input = String::new();
    //     num = 0;

    //     if ch == '1' {
    //         println!("Enter node value: ");
    //         io::stdin()
    //             .read_line(&mut tree_value_input)
    //             .expect("Couldn't read value from stdin");

    //         num = tree_value_input.trim().parse().expect("NaN");
    //     }

    //     tree_root.add_node(num, pos+2);
    // }

    println!("{}", tree_root.print());

    ExitCode::SUCCESS
}
