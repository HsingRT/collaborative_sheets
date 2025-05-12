use collaborative_sheets::{
    access_control::{AccessControlManager, AccessRight},
    sheet_management::SheetManager,
    user_management::UserManager,
};

#[test]
fn test_create_and_check_sheet() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control = AccessControlManager::new();

    user_manager.create_user("alice");
    let alice_id = user_manager.get_user_id("alice").unwrap();

    sheet_manager.create_sheet(alice_id, "math");
    let sheet_id = sheet_manager.get_sheet_id("math").unwrap();

    assert!(sheet_manager.get_sheet(sheet_id).is_some());

    access_control.set_access_right(sheet_id, alice_id, AccessRight::Editable);
    assert!(access_control.check_access_editable(sheet_id, alice_id));
}

#[test]
fn test_change_sheet_value_with_editable_access() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control = AccessControlManager::new();

    user_manager.create_user("bob");
    let bob_id = user_manager.get_user_id("bob").unwrap();

    sheet_manager.create_sheet(bob_id, "budget");
    let sheet_id = sheet_manager.get_sheet_id("budget").unwrap();
    access_control.set_access_right(sheet_id, bob_id, AccessRight::Editable);

    let sheet = sheet_manager.get_sheet_mut(sheet_id).unwrap();
    assert_eq!(sheet.content[0][0], 0.0);

    let value = collaborative_sheets::arithmetic::evaluate_expression("10 + 20").unwrap();
    sheet.content[0][0] = value as f32;

    assert_eq!(sheet.content[0][0], 30.0);
}

#[test]
fn test_read_access_only_blocks_edit() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control = AccessControlManager::new();

    user_manager.create_user("carol");
    let carol_id = user_manager.get_user_id("carol").unwrap();

    sheet_manager.create_sheet(carol_id, "plan");
    let sheet_id = sheet_manager.get_sheet_id("plan").unwrap();
    access_control.set_access_right(sheet_id, carol_id, AccessRight::ReadOnly);

    assert!(!access_control.check_access_editable(sheet_id, carol_id));
    assert!(access_control.check_access_read_only(sheet_id, carol_id));
}

#[test]
fn test_owner_can_delete_sheet() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control = AccessControlManager::new();

    user_manager.create_user("dave");
    let dave_id = user_manager.get_user_id("dave").unwrap();

    sheet_manager.create_sheet(dave_id, "travel");
    let sheet_id = sheet_manager.get_sheet_id("travel").unwrap();
    access_control.set_access_right(sheet_id, dave_id, AccessRight::Editable);

    assert!(sheet_manager.is_owner(dave_id, sheet_id));
    assert!(sheet_manager.get_sheet(sheet_id).is_some());

    sheet_manager.delete_sheet(sheet_id);
    access_control.remove_sheet(sheet_id);

    assert!(sheet_manager.get_sheet(sheet_id).is_none());
    assert!(!access_control.check_access_read_only(sheet_id, dave_id));
}

#[test]
fn test_share_sheet_to_other_user() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control = AccessControlManager::new();

    user_manager.create_user("eve");
    user_manager.create_user("frank");

    let eve_id = user_manager.get_user_id("eve").unwrap();
    let frank_id = user_manager.get_user_id("frank").unwrap();

    sheet_manager.create_sheet(eve_id, "project");
    let sheet_id = sheet_manager.get_sheet_id("project").unwrap();

    assert!(sheet_manager.is_owner(eve_id, sheet_id));

    access_control.set_access_right(sheet_id, frank_id, AccessRight::ReadOnly);
    assert!(access_control.check_access_read_only(sheet_id, frank_id));
    assert!(!access_control.check_access_editable(sheet_id, frank_id));
}