/*
    Arithmetic expression evaluator
    Given an arithmetic expression, evaluate it and return the result.
    The expression can contain the following operators: +, -, *, /
    The expression can contain parentheses to change the order of operations.
    The expression can contain floating point numbers.
    The expression can contain spaces between numbers and operators.
*/
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    match expression {
        "" => Err("Empty expression".to_string()),
        _ => {
            let tokens = parse_expression(expression)?;
            let rpn_tokens = infix_to_rpn(tokens)?;
            compute(rpn_tokens)
        }
    }
}

// Parse the expression into tokens
fn parse_expression(expression: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    for c in expression.chars() {
        if c.is_digit(10) || c == '.' {
            number.push(c);
        } else if c.is_whitespace() {
            continue;
        } else {
            if !number.is_empty() {
                tokens.push(number.clone());
                number.clear();
            }
            tokens.push(c.to_string());
        }
    }

    if !number.is_empty() {
        tokens.push(number);
    }

    Ok(tokens)
}

// Convert infix expression to Reverse Polish Notation (RPN)
fn infix_to_rpn(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    let precedence = |op: &str| -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    };

    for token in tokens {
        if let Ok(_) = token.parse::<f64>() {
            output.push(token);
        } else if token == "(" {
            operators.push(token);
        } else if token == ")" {
            while let Some(op) = operators.pop() {
                if op == "(" {
                    break;
                } else {
                    output.push(op);
                }
            }
        } else {
            while let Some(op) = operators.last() {
                if precedence(op) >= precedence(&token) {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token);
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}

// Compute the result of the RPN expression
fn compute(tokens: Vec<String>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().ok_or("Invalid expression".to_string())?;
                let a = stack.pop().ok_or("Invalid expression".to_string())?;
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().ok_or("Invalid expression".to_string())?;
                let a = stack.pop().ok_or("Invalid expression".to_string())?;
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().ok_or("Invalid expression".to_string())?;
                let a = stack.pop().ok_or("Invalid expression".to_string())?;
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().ok_or("Invalid expression".to_string())?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                let a = stack.pop().ok_or("Invalid expression".to_string())?;
                stack.push(a / b);
            }
            _ => {
                let number = token.parse::<f64>().map_err(|_| "Invalid number".to_string())?;
                stack.push(number);
            }
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression".to_string())
    }
}

#[cfg(test)]
mod tests;