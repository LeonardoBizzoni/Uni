use std::io;
use tableau::Formula;

fn main() -> io::Result<()> {
    let mut input_buffer = String::new();

    println!("Enter formula:");
    if let Err(err) = io::stdin().read_line(&mut input_buffer) {
	eprintln!("[ERROR] failed to read from stdin.");
	return Err(err);
    }

    let input_buffer: String = input_buffer.trim().to_string();

    if Formula::check_syntax(&input_buffer) {
	let mut fbf = Formula::new(input_buffer);
	fbf.find_variables(&fbf.syntax_tree.clone(), 0);
	fbf.evaluate_formula();
	
	println!("{}", fbf.syntax_tree);
	print!("Modelli: ");
	for ele in fbf.models.iter() {
	    print!("{} ", ele);
	}
	print!("\nContromodelli: ");
	for ele in fbf.contradictions.iter() {
	    print!("{} ", ele);
	}

	println!("\nLa formula è: {}", fbf.get_type());
    } else {
	eprintln!("Invalid formula");
    }

    Ok(())
}
