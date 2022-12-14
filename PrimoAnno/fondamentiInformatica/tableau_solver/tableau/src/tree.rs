use crate::ElementType;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tree {
    root: char,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new(formula: &str) -> Self {
        let op_stack = Self::create_stack(&formula);

        match Self::get_root(&op_stack) {
            Some((root, idx)) => {
                let mut left  = &formula[..idx];
                let mut right = &formula[idx+1..];

		if left.chars().nth(0) == Some('(') {
		    left = &formula[1..idx];
		}
		if right.chars().nth(0) == Some('(') {
		    right = &formula[idx+1..formula.len()-1];
		}

                Self {
                    root,
                    left: Some(Box::from(Tree::new(left))),
                    right: Some(Box::from(Tree::new(right))),
                }
            }
            None => Self {
                root: formula.chars().nth(0).unwrap(),
                left: None,
                right: None,
            },
        }
    }

    fn get_root(op_stack: &[ElementType]) -> Option<(char, usize)> {
        // Se il primo elemento è un operatore unario allora quello è l'operatore principale.
        // Se il primo elemento è un atomo allora il prossimo elemento deve essere un operatore binario ed l'operatore principale
        // Se il primo elemento è una parentesi richiama il metodo con il contenuto della parentesi

        if matches!(op_stack[0], ElementType::Unary) {
            return Some(('!', 0));
        } else if matches!(op_stack[0], ElementType::Atom(_)) {
            if op_stack.len() >= 2 {
                match op_stack[1] {
                    ElementType::Binary('&') => return Some(('&', 1)),
                    ElementType::Binary('|') => return Some(('|', 1)),
                    ElementType::Binary('>') => return Some(('>', 1)),
                    ElementType::Binary('-') => return Some(('-', 1)),
                    _ => {}
                }
            }

            return None;
        } else if matches!(op_stack[0], ElementType::NestOpen) {
            let mut nest_stack: Vec<usize> = Vec::new();
            let mut root_idx: isize = -1;

            for (idx, op) in op_stack.iter().enumerate() {
                match op {
                    ElementType::NestOpen => nest_stack.push(1),
                    ElementType::NestClose => {
                        nest_stack.pop();

                        if nest_stack.len() == 0 {
                            root_idx = idx as isize;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if root_idx != -1 {
                let root_idx: usize = root_idx as usize + 1;

                match op_stack[root_idx] {
                    ElementType::Binary('&') => return Some(('&', root_idx)),
                    ElementType::Binary('|') => return Some(('|', root_idx)),
                    ElementType::Binary('>') => return Some(('>', root_idx)),
                    ElementType::Binary('-') => return Some(('-', root_idx)),
                    _ => {}
                }
            }

            return None;
        }

        None
    }

    fn create_stack(formula: &str) -> Vec<ElementType> {
        let mut op_stack: Vec<ElementType> = Vec::new();

        for ch in formula.chars() {
            match ch {
                '!' => op_stack.push(ElementType::Unary),

                '|' | '&' | '>' | '-' => op_stack.push(ElementType::Binary(ch)),

                '(' | '[' | '{' => op_stack.push(ElementType::NestOpen),
                ')' | ']' | '}' => op_stack.push(ElementType::NestClose),

                _ => op_stack.push(ElementType::Atom(ch)),
            }
        }

        op_stack
    }
}
