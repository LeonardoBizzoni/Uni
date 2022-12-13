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
	let fbf = Formula::new(input_buffer);
    } else {
	eprintln!("Invalid formula");
    }

    Ok(())
}
