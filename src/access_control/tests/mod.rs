use super::*;

#[test]
fn test_set_and_get_access_right() {
    let mut manager = AccessControlManager::new();
    manager.set_access_right(1, 100, AccessRight::Editable);

    assert!(manager.check_access_editable(1, 100));
    assert!(!manager.check_access_read_only(1, 100));
}

#[test]
fn test_update_access_right() {
    let mut manager = AccessControlManager::new();
    manager.set_access_right(1, 101, AccessRight::ReadOnly);
    manager.set_access_right(1, 101, AccessRight::Editable);

    assert!(manager.check_access_editable(1, 101));
    assert!(!manager.check_access_read_only(1, 101));
}

#[test]
fn test_share_sheet() {
    let mut manager = AccessControlManager::new();
    manager.share_sheet(2, 200, AccessRight::ReadOnly);

    assert!(manager.check_access_read_only(2, 200));
}

#[test]
fn test_unshared_sheet() {
    let mut manager = AccessControlManager::new();
    manager.set_access_right(3, 300, AccessRight::Editable);
    manager.unshared_sheet(3, 300);

    assert!(!manager.check_access_editable(3, 300));
    assert!(!manager.check_access_read_only(3, 300));
}

#[test]
fn test_remove_sheet() {
    let mut manager = AccessControlManager::new();
    manager.set_access_right(4, 400, AccessRight::Editable);
    manager.remove_sheet(4);

    assert!(!manager.check_access_editable(4, 400));
}

#[test]
fn test_check_access_rights() {
    let mut manager = AccessControlManager::new();
    manager.set_access_right(5, 500, AccessRight::ReadOnly);

    assert!(manager.check_access_read_only(5, 500));
    assert!(!manager.check_access_editable(5, 500));
}