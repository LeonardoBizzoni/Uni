#[derive(Debug, Clone)]
pub struct Atom {
    pub name: char,
    pub value: bool,
}

impl Atom {
    #[inline]
    pub fn new(name: char, value: bool) -> Self {
	Self { name, value }
    }

    pub fn set_value(&mut self, value: bool) {
	self.value = value;
    }
}
