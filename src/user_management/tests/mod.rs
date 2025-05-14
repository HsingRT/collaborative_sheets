use super::*;

#[test]
fn test_create_user_and_get_id() {
    let mut manager = UserManager::new();

    manager.create_user("alice");
    manager.create_user("bob");

    let alice_id = manager.get_user_id("alice");
    let bob_id = manager.get_user_id("bob");
    let not_found = manager.get_user_id("charlie");

    assert!(alice_id.is_some());
    assert!(bob_id.is_some());
    assert_ne!(alice_id, bob_id);
    assert!(not_found.is_none());
}

#[test]
fn test_user_exists() {
    let mut manager = UserManager::new();
    assert!(!manager.user_exists("alice"));

    manager.create_user("alice");
    assert!(manager.user_exists("alice"));
}

#[test]
fn test_delete_user() {
    let mut manager = UserManager::new();
    manager.create_user("alice");
    assert!(manager.user_exists("alice"));

    let deleted = manager.delete_user("alice");
    assert!(deleted);
    assert!(!manager.user_exists("alice"));

    let deleted_again = manager.delete_user("alice");
    assert!(!deleted_again);
}

#[test]
fn test_duplicate_usernames_allowed() {
    let mut manager = UserManager::new();

    manager.create_user("duplicate");
    manager.create_user("duplicate");

    let count = manager
        .users
        .iter()
        .filter(|u| u.username == "duplicate")
        .count();

    assert_eq!(count, 2); // Currently, no uniqueness enforced
}