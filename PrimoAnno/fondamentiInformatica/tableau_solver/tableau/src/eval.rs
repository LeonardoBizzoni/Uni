use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Eval {
    pub assignment: Vec<char>,
}

impl Eval {
    #[inline]
    pub fn new(assignment: Vec<char>) -> Self {
	Self { assignment }
    }
}

impl Display for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	let mut res: String = String::new();

	self.assignment.iter().for_each(|assign| {
	    res.push(*assign);
	});

	write!(f, "{{{}}}", res)
    }
}
