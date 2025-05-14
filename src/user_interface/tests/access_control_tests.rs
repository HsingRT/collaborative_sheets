use crate::user_interface::access_control::{
    collaborate_with_input, unshared_sheet_with_input,
};
use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::{AccessControlManager, AccessRight};

#[test]
fn test_collaborate_and_change_access() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    user_manager.create_user("alice");
    user_manager.create_user("bob");

    let alice_id = user_manager.get_user_id("alice").unwrap();
    sheet_manager.create_sheet(alice_id, "sheet1");
    let sheet_id = sheet_manager.get_sheet_id("sheet1").unwrap();

    // Ensure alice has editable access before she tries to share
    access_control_manager.set_access_right(sheet_id, alice_id, AccessRight::Editable);
    assert!(access_control_manager.check_access_editable(sheet_id, alice_id));

    collaborate_with_input("alice sheet1 bob", &user_manager, &sheet_manager, &mut access_control_manager);

    let bob_id = user_manager.get_user_id("bob").unwrap();
    assert!(access_control_manager.check_access_editable(sheet_id, bob_id));
}

#[test]
fn test_unshare_sheet() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    user_manager.create_user("alice");
    user_manager.create_user("bob");

    let alice_id = user_manager.get_user_id("alice").unwrap();
    sheet_manager.create_sheet(alice_id, "sheet2");
    let sheet_id = sheet_manager.get_sheet_id("sheet2").unwrap();
    access_control_manager.set_access_right(sheet_id, alice_id, AccessRight::Editable);

    access_control_manager.share_sheet(sheet_id, user_manager.get_user_id("bob").unwrap(), AccessRight::Editable);
    assert!(access_control_manager.check_access_editable(sheet_id, user_manager.get_user_id("bob").unwrap()));

    // Ensure owner can revoke access from shared user
    unshared_sheet_with_input("alice sheet2 bob", &user_manager, &sheet_manager, &mut access_control_manager);
    assert!(!access_control_manager.check_access_editable(sheet_id, user_manager.get_user_id("bob").unwrap()));
}