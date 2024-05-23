pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    match expression { 
        "" => Err("Empty expression".to_string()),
        _ => {
            let tokens = parse_expression(expression)?;
            compute(tokens)
        }
    }
}

// Parse the expression into tokens
fn parse_expression(expression: &str) -> Result<Vec<&str>, String> {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    Ok(tokens)
}

// Compute the result of the expression
fn compute(tokens: Vec<&str>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        match token {
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