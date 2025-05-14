use super::*;

#[test]
fn test_evaluate_expression() {
    assert_eq!(evaluate_expression("3 + 5").unwrap(), 8.0);
    assert_eq!(evaluate_expression("10 + 2 * 6").unwrap(), 22.0);
    assert_eq!(evaluate_expression("100 * 2 + 12").unwrap(), 212.0);
    assert_eq!(evaluate_expression("100 * ( 2 + 12 )").unwrap(), 1400.0);
    assert_eq!(evaluate_expression("100 * ( 2 + 12 ) / 14").unwrap(), 100.0);
    assert_eq!(evaluate_expression("1.5 * 3").unwrap(), 4.5);
}

#[test]
fn test_evaluate_invalid_expression() {
    assert!(evaluate_expression("3 +").is_err());
    assert!(evaluate_expression("10 / 0").is_err());
}