pub struct Node {
    pub value: u8,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    pub pos: usize,
}
