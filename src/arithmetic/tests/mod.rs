use super::*;

// === Tests for evaluate_expression ===

#[test]
fn test_addition() {
    assert_eq!(evaluate_expression("3 + 5").unwrap(), 8.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(evaluate_expression("10 - 4").unwrap(), 6.0);
}

#[test]
fn test_multiplication_and_precedence() {
    assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
}

#[test]
fn test_division() {
    assert_eq!(evaluate_expression("8 / 2").unwrap(), 4.0);
}

#[test]
fn test_parentheses_override_precedence() {
    assert_eq!(evaluate_expression("2 * (3 + 4)").unwrap(), 14.0);
}

#[test]
fn test_floating_point_result() {
    assert_eq!(evaluate_expression("1.5 * 2").unwrap(), 3.0);
}

#[test]
fn test_expression_with_spaces() {
    assert_eq!(evaluate_expression("  6 -  1 ").unwrap(), 5.0);
}

#[test]
fn test_empty_expression_should_error() {
    assert!(evaluate_expression("").is_err());
}

#[test]
fn test_division_by_zero_should_error() {
    assert!(evaluate_expression("10 / 0").is_err());
}

#[test]
fn test_unexpected_end_should_error() {
    assert!(evaluate_expression("7 +").is_err());
}

#[test]
fn test_invalid_token_should_error() {
    assert!(evaluate_expression("2 + x").is_err());
}

#[test]
fn test_unmatched_parenthesis_should_error() {
    assert!(evaluate_expression("(2 + 3").is_err());
}

#[test]
fn test_consecutive_operators_should_error() {
    assert!(evaluate_expression("4 + * 2").is_err());
}

#[test]
fn test_extra_operators_should_error() {
    assert!(evaluate_expression("5 + 2 -").is_err());
}

// === parse_expression ===

#[test]
fn test_parse_expression_with_mixed_tokens() {
    let tokens = parse_expression("3.14 + (2 - 1)").unwrap();
    assert_eq!(tokens, vec!["3.14", "+", "(", "2", "-", "1", ")"]);
}

// === infix_to_rpn ===

#[test]
fn test_infix_to_rpn_with_subtraction_and_parentheses() {
    let rpn = infix_to_rpn(vec!["(".into(), "5".into(), "-".into(), "2".into(), ")".into(), "*".into(), "3".into()]).unwrap();
    assert_eq!(rpn, vec!["5", "2", "-", "3", "*"]);
}

// === compute ===

#[test]
fn test_compute_valid_subtraction_rpn() {
    let result = compute(vec!["10".into(), "3".into(), "-".into()]).unwrap();
    assert_eq!(result, 7.0);
}

#[test]
fn test_compute_invalid_operand_count() {
    let result = compute(vec!["5".into(), "+".into()]);
    assert!(result.is_err());
}

#[test]
fn test_compute_invalid_token() {
    let result = compute(vec!["1".into(), "abc".into(), "+".into()]);
    assert!(result.is_err());
}

#[test]
fn test_compute_stack_length_not_one_should_error() {
    // Equivalent to evaluating "3 4" without operator: both are pushed but never used
    let rpn_tokens = vec!["3".to_string(), "4".to_string()];
    let result = compute(rpn_tokens);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid expression");
}