pub struct Tree {
    root: Box<String>,
    left: Option<Box<String>>,
    right: Option<Box<String>>,
}

impl Tree {
    pub fn new(value: String) -> Self {
	Self { root: Box::from(value), left: None, right: None }
    }
}
