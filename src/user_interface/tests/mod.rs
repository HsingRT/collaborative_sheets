pub mod access_control_tests;
pub mod sheet_management_tests;
pub mod user_management_tests;

use crate::user_interface::{parse_command, dispatch_command, DispatchResult};
use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;

#[test]
fn test_parse_command_valid_inputs() {
    assert_eq!(parse_command("1"), 1);
    assert_eq!(parse_command("10"), 10);
    assert_eq!(parse_command(" 3 "), 3);
}

#[test]
fn test_parse_command_invalid_inputs() {
    assert_eq!(parse_command(""), 0);
    assert_eq!(parse_command("invalid"), 0);
    assert_eq!(parse_command("99abc"), 0);
}

#[test]
fn test_dispatch_command_returns_exit_on_command_10() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    let result = dispatch_command(
        10,
        &mut user_manager,
        &mut sheet_manager,
        &mut access_control_manager,
    );

    assert!(matches!(result, DispatchResult::Exit));
}