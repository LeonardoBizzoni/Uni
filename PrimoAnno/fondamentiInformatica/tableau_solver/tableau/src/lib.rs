#[derive(Debug, PartialEq, Clone, Copy)]
enum LogicOperator {
    And,
    Or,
    Not,
    Imp,
    Iff,
    NestOpen,
    NestClose,
}

pub struct Formula<'a> {
    _formula: &'a str,
}

impl Formula<'_> {
    pub fn check_syntax(formula: &str) -> bool {
        let mut op_stack: Vec<LogicOperator> = Vec::new();
        let mut var_stack: Vec<char> = Vec::new();
        let mut skip_idx: usize = 0;
        let mut prev_was_not = false;

        for (idx, ch) in formula.chars().enumerate() {
            if idx >= skip_idx {
		println!("Formula: {}, current: {}", formula, ch);
                match ch {
                    '!' => {
                        op_stack.push(LogicOperator::Not);
                        prev_was_not = true;
                    }
                    '|' => {if !prev_was_not {op_stack.push(LogicOperator::Or)}  else {return false;}},
                    '&' => {if !prev_was_not {op_stack.push(LogicOperator::And)} else {return false;}},
                    '>' => {if !prev_was_not {op_stack.push(LogicOperator::Imp)} else {return false;}},
                    '-' => {if !prev_was_not {op_stack.push(LogicOperator::Iff)} else {return false;}},

                    '(' => {
			op_stack.push(LogicOperator::NestOpen);
                        if let Some(closing) = formula[idx + 1..].find(')') {
                            skip_idx = closing + idx + 1;
                            if Self::check_syntax(&formula[idx+1..skip_idx]) {
                                var_stack.push('_');
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    ')' => {op_stack.push(LogicOperator::NestClose); prev_was_not = false},

                    _ => {var_stack.push(ch); prev_was_not = false;},
                }
            }
        }

	if prev_was_not {
	    return false;
	}

	println!("Formula: {}:", formula);
        Self::check_stacks(&mut op_stack, &mut var_stack)
    }

    fn check_stacks(op_stack: &mut Vec<LogicOperator>, var_stack: &mut Vec<char>) -> bool {
        while !op_stack.is_empty() {
	    println!("\t{:?}", op_stack);
            match op_stack.pop().unwrap() {
                LogicOperator::Not => {
                    if let None = var_stack.pop() {
                        return false;
                    }
                    var_stack.push('_');
                }
                LogicOperator::And
                | LogicOperator::Iff
                | LogicOperator::Imp
                | LogicOperator::Or => {
                    if let None = var_stack.pop() {
                        return false;
                    }
                    if let None = var_stack.pop() {
                        return false;
                    }

                    var_stack.push('_');
                },
		LogicOperator::NestOpen => return false,
		LogicOperator::NestClose => {
		    if let Some(open_pos) = op_stack.iter().position(|&op| op == LogicOperator::NestOpen) {
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
