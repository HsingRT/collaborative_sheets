use crate::user_interface::user_management::create_user_with_name;
use crate::user_management::UserManager;

/// Verify that `create_user_with_name` correctly adds a user to the manager
#[test]
fn test_create_user_with_name_adds_user() {
    let mut user_manager = UserManager::new();

    create_user_with_name("shawn", &mut user_manager);

    assert!(user_manager.user_exists("shawn"));
}