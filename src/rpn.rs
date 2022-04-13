use std::cmp::{PartialOrd, Ordering};

pub fn convert_infix_to_rpn(infix_formula_token: Vec<FormulaElement>) -> Vec<FormulaElement> {
    let mut rpn_tokens: Vec<FormulaElement> = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();
    for token in infix_formula_token {
        match token {
            FormulaElement::Operand(_) => rpn_tokens.push(token),
            FormulaElement::Operator(operator) => {
                if operator_stack.is_empty() {
                    operator_stack.push(operator.clone())
                } else {
                    let op1 = operator_stack.last().unwrap();
                    if &operator > op1 {
                        operator_stack.push(operator.clone());
                    } else {
                        for stacked_operator in operator_stack.iter() {
                            rpn_tokens.push(FormulaElement::Operator(stacked_operator.clone()));
                        }
                        rpn_tokens.push(FormulaElement::Operator(operator.clone()));
                    }
                }
            }
        }
    }
    println!("{:?}", operator_stack);
    for op in operator_stack.iter().rev() {
        rpn_tokens.push(FormulaElement::Operator(op.clone()));
    }

    return rpn_tokens;
}

pub fn calculate_rpn(formula_tokens: Vec<FormulaElement>) -> Result<f32, String> {
    let mut stack:Vec<f32> = Vec::new();
    for token in formula_tokens {
        match token {
            FormulaElement::Operator(operator) => {
                if stack.len() < 2 {
                    return Err("stack error".to_string())
                }
                let operand2: f32 = stack.pop().expect("unexpected error");
                let operand1: f32 = stack.pop().expect("unexpected error");
                let result = match operator {
                    Operator::Add => operand1 + operand2,
                    Operator::Sub => operand1 - operand2,
                    Operator::Mul => operand1 * operand2,
                    Operator::Div => operand1 / operand2
                };
                stack.push(result);
            },
            FormulaElement::Operand(value) => stack.push(value)
        }
    }
    Ok(stack.pop().expect("unexpected error"))
}

#[test]
fn test1() {
    // 4 + 2 * 3 -> 4 2 3 + * -> 10
    let s = convert_infix_to_rpn(
        vec![
            FormulaElement::Operand(4.0), 
            FormulaElement::Operator(Operator::Add), 
            FormulaElement::Operand(2.0),
            FormulaElement::Operator(Operator::Mul), 
            FormulaElement::Operand(3.0),
            ]);
    println!("{:?}", s);
    let result = calculate_rpn(s).expect("error");

    assert_eq!(result, 10.0)
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Operator::*;
        match (self, other) {
            (Add, Add) => Some(Ordering::Equal),
            (Sub, Add) => Some(Ordering::Equal),
            (Sub, Sub) => Some(Ordering::Equal),
            (Div, Div) => Some(Ordering::Equal),
            (Sub, Div) => Some(Ordering::Less),
            (Mul, Add) => Some(Ordering::Greater),
            (Mul, Sub) => Some(Ordering::Greater),
            (Mul, Mul) => Some(Ordering::Equal),
            (Div, Sub) => Some(Ordering::Greater),
            (Div, Mul) => Some(Ordering::Equal),
            (Add, Sub) => Some(Ordering::Equal),
            (Add, Mul) => Some(Ordering::Less),
            (Add, Div) => Some(Ordering::Less),
            (Sub, Mul) => Some(Ordering::Less),
            (Mul, Div) => Some(Ordering::Equal),
            (Div, Add) => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormulaElement {
    Operator(Operator),
    Operand(f32)
}