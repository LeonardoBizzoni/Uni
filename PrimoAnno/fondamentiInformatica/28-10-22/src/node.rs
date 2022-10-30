#[derive(Clone)]
pub struct Node {
    pub value: u8,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    pub pos: usize,
}

impl Node {
    pub fn new(value: u8, pos: usize) -> Option<Box<Node>> {
	Some(Box::new(Node {
            value,
            left: None,
            right: None,
            pos,
        }))
    }
}
