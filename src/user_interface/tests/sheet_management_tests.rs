use crate::user_interface::sheet_management::{
    create_sheet_with_input, change_sheet_value_with_input, delete_sheet_with_input,
};
use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;

#[test]
fn test_create_sheet_and_check_permissions() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    user_manager.create_user("alice");
    create_sheet_with_input(
        "alice sheet1",
        &mut user_manager,
        &mut sheet_manager,
        &mut access_control_manager,
    );

    let sheet_id = sheet_manager.get_sheet_id("sheet1").unwrap();
    let user_id = user_manager.get_user_id("alice").unwrap();
    assert!(sheet_manager.is_owner(user_id, sheet_id));
    assert!(access_control_manager.check_access_editable(sheet_id, user_id));
}

#[test]
fn test_change_sheet_value() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    user_manager.create_user("alice");
    create_sheet_with_input("alice sheet2", &mut user_manager, &mut sheet_manager, &mut access_control_manager);

    change_sheet_value_with_input(
        "alice sheet2 2 2 3+2*2",
        &user_manager,
        &mut sheet_manager,
        &access_control_manager,
    );

    let sheet_id = sheet_manager.get_sheet_id("sheet2").unwrap();
    let sheet = sheet_manager.get_sheet_mut(sheet_id).unwrap();
    assert_eq!(sheet.content[1][1], 7.0); // (2,2) 對應 index 為 (1,1)
}

#[test]
fn test_delete_sheet_permission_check() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    user_manager.create_user("alice");
    user_manager.create_user("bob");

    create_sheet_with_input("alice sheet3", &mut user_manager, &mut sheet_manager, &mut access_control_manager);

    let before = sheet_manager.get_sheet_id("sheet3").is_some();
    assert!(before);

    // bob (not owner) tries to delete
    delete_sheet_with_input("bob sheet3", &user_manager, &mut sheet_manager, &mut access_control_manager);

    let still_exists = sheet_manager.get_sheet_id("sheet3").is_some();
    assert!(still_exists);

    // alice (owner) deletes
    delete_sheet_with_input("alice sheet3", &user_manager, &mut sheet_manager, &mut access_control_manager);
    let deleted = sheet_manager.get_sheet_id("sheet3").is_none();
    assert!(deleted);
}