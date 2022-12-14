use crate::tree::Tree;
use crate::LogicType;

pub struct Formula {
    pub formula: String,
    pub syntax_tree: Tree,
}

impl Formula {
    pub fn new(formula: String) -> Self {
        Formula {
            formula: formula.clone(),
            syntax_tree: Tree::new(&formula),
        }
    }

    pub fn check_syntax(formula: &String) -> bool {
        let mut op_stack: Vec<LogicType> = Vec::new();
        let mut var_stack: Vec<char> = Vec::new();
        let mut prev_was_not = false;

        for ch in formula.chars() {
            match ch {
                '!' => {
                    op_stack.push(LogicType::Unary);
                    prev_was_not = true;
                }

                '|' | '&' | '>' | '-' => {
                    if !prev_was_not {
                        op_stack.push(LogicType::Binary)
                    } else {
                        return false;
                    }
                }

                '(' | '[' | '{' => op_stack.push(LogicType::NestOpen),
                ')' | ']' | '}' => {
                    op_stack.push(LogicType::NestClose);
                    prev_was_not = false
                }

                _ => {
                    var_stack.push(ch);
                    prev_was_not = false;
                }
            }
        }

        if prev_was_not {
            return false;
        }

        Self::check_stacks(&mut op_stack, &mut var_stack)
    }

    fn check_stacks(op_stack: &mut Vec<LogicType>, var_stack: &mut Vec<char>) -> bool {
        while !op_stack.is_empty() {
            match op_stack.pop().unwrap() {
                LogicType::Unary => {
                    if let None = var_stack.pop() {
                        return false;
                    }
                    var_stack.push('_');
                }
                LogicType::Binary => {
                    if let None = var_stack.pop() {
                        return false;
                    }
                    if let None = var_stack.pop() {
                        return false;
                    }

                    var_stack.push('_');
                }
                LogicType::NestOpen => return false,
                LogicType::NestClose => {
                    if let Some(open_pos) =
                        op_stack.iter().rposition(|&op| op == LogicType::NestOpen)
                    {
                        op_stack.remove(open_pos);
                    } else {
                        return false;
                    }
                }
            }
        }

        if var_stack.len() != 1 {
            return false;
        }

        true
    }
}
