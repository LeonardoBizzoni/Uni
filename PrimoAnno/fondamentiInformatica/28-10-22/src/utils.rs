use regex::Regex;

pub fn check_syntax(binary_str: &str) -> Result<(), &'static str> {
    let reg = Regex::new("[01]").unwrap();

    if binary_str.chars().nth(0) == Some('1') && reg.replace_all(binary_str, "").len() == 0 {
	return Ok(());
    }

    Err("Invalid syntax")
}
