use std::collections::HashMap;

use crate::atom::Atom;
use crate::eval::Eval;
use crate::tree::Tree;
use crate::LogicType;

#[derive(Clone)]
pub struct Formula {
    pub formula: String,
    pub syntax_tree: Tree,
    pub variable_map: HashMap<usize, Atom>,
    pub models: Vec<Eval>,
    pub contradictions: Vec<Eval>,
}

impl Formula {
    pub fn new(formula: String) -> Self {
        Formula {
            formula: formula.clone(),
            syntax_tree: Tree::new(&formula),
            variable_map: HashMap::new(),
            models: Vec::new(),
            contradictions: Vec::new(),
        }
    }

    pub fn evaluate_formula(&mut self) {
        let max_bit: i32 = (2 as i32).pow(self.variable_map.len() as u32);

        for assignment in 0..max_bit {
            let bits = Self::to_bin(assignment, self.variable_map.len());

            bits.iter()
                .enumerate()
                .for_each(|(idx, value)| match *value {
                    0 => self.variable_map.get_mut(&idx).unwrap().set_value(false),
                    1 => self.variable_map.get_mut(&idx).unwrap().set_value(true),
                    _ => {}
                });

            println!("Assignment: {:?}", self.variable_map.values());
            if Self::evaluate_assignment(&self.syntax_tree, &self.variable_map) {
                self.models.push(Eval {
                    assignment: self
                        .variable_map
                        .clone()
                        .into_iter()
                        .filter(|(_, eval)| eval.value)
                        .map(|(_, eval)| eval.name)
                        .collect::<Vec<char>>(),
                });
            } else {
                self.contradictions.push(Eval {
                    assignment: self
                        .variable_map
                        .clone()
                        .into_iter()
                        .filter(|(_, eval)| eval.value)
                        .map(|(_, eval)| eval.name)
                        .collect::<Vec<char>>(),
                });
            }
        }
    }

    fn evaluate_assignment(tree: &Tree, assignment: &HashMap<usize, Atom>) -> bool {
        if tree.root.is_alphabetic() {
            return assignment
                .values()
                .find(|eval| eval.name == tree.root)
                .unwrap()
                .value;
        } else if tree.root == '!' {
            return !Self::evaluate_assignment(tree.right.as_ref().unwrap(), assignment);
        }

        let left = Self::evaluate_assignment(tree.left.as_ref().unwrap(), assignment);
        let right = Self::evaluate_assignment(tree.right.as_ref().unwrap(), assignment);

        match tree.root {
            '&' => left && right,
            '|' => left || right,
            '>' => !left || right,
            '-' => left == right,
            _ => panic!("No idea"),
        }
    }

    fn to_bin(mut n: i32, capacity: usize) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(capacity);

        while n > 0 {
            res.push(n % 2);
            n /= 2;
        }

        res
    }
    pub fn find_variables(&mut self, tree: &Tree, mut idx: usize) -> usize {
        if tree.left.is_none() && tree.right.is_none() {
            if !self
                .variable_map
                .values()
                .any(|eval| eval.name == tree.root)
            {
                self.variable_map.insert(
                    idx,
                    Atom {
                        name: tree.root,
                        value: false,
                    },
                );
                return idx + 1;
            }

            return idx;
        }

        if tree.left.is_some() {
            idx = self.find_variables(tree.left.as_ref().unwrap(), idx);
        }
        if tree.right.is_some() {
            idx = self.find_variables(tree.right.as_ref().unwrap(), idx);
        }

        idx
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
