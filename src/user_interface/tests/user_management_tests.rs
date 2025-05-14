use crate::user_interface::user_management::{create_user_with_name, delete_user_with_name};
use crate::user_management::UserManager;

#[test]
fn test_create_user_with_name_adds_user() {
    let mut user_manager = UserManager::new();

    create_user_with_name("alice", &mut user_manager);
    assert!(user_manager.user_exists("alice"));

    // Duplicate usernames should be ignored by the UI layer
    create_user_with_name("alice", &mut user_manager);
    let exists = user_manager.get_user_id("alice").is_some();
    assert!(exists);
}

#[test]
fn test_delete_user_with_name_removes_user() {
    let mut user_manager = UserManager::new();

    create_user_with_name("bob", &mut user_manager);
    assert!(user_manager.user_exists("bob"));

    delete_user_with_name("bob", &mut user_manager);
    assert!(!user_manager.user_exists("bob"));
}

#[test]
fn test_delete_user_with_name_nonexistent() {
    let mut user_manager = UserManager::new();

    // Deleting a non-existent user should not panic
    delete_user_with_name("charlie", &mut user_manager);
    assert!(!user_manager.user_exists("charlie"));
}
