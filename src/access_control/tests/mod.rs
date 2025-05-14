use super::*;

/// Shared constants used across test cases
const SHEET_A: u32 = 1;
const SHEET_B: u32 = 2;
const ALICE:  u32 = 100;
const BOB:    u32 = 200;

#[test]
fn create_entry_with_set_access_right() {
    // New sheet → entry should be created with specified right
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::Editable);

    assert!(mgr.check_access_editable(SHEET_A, ALICE));
    assert!(!mgr.check_access_read_only(SHEET_A, ALICE));
}

#[test]
fn update_existing_entry() {
    // Updating the same user on the same sheet should override, not duplicate
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::ReadOnly);
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::Editable);

    assert!(mgr.check_access_editable(SHEET_A, ALICE));
}

#[test]
fn share_sheet_is_alias_for_set_access_right() {
    let mut mgr = AccessControlManager::new();
    mgr.share_sheet(SHEET_A, BOB, AccessRight::ReadOnly);

    assert!(mgr.check_access_read_only(SHEET_A, BOB));
}

#[test]
fn unshare_removes_user_but_preserves_others() {
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::Editable);
    mgr.set_access_right(SHEET_A, BOB,   AccessRight::ReadOnly);

    mgr.unshared_sheet(SHEET_A, ALICE);

    // Alice is gone; Bob is unaffected
    assert!(!mgr.check_access_editable(SHEET_A, ALICE));
    assert!(mgr.check_access_read_only (SHEET_A, BOB));
}

#[test]
fn remove_sheet_clears_everything_for_that_sheet_only() {
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::Editable);
    mgr.set_access_right(SHEET_B, ALICE, AccessRight::ReadOnly);

    mgr.remove_sheet(SHEET_A);

    assert!(!mgr.check_access_editable(SHEET_A, ALICE));
    assert!(mgr.check_access_read_only (SHEET_B, ALICE)); // other sheet untouched
}

#[test]
fn unknown_sheet_or_user_returns_false_in_checks() {
    let mgr = AccessControlManager::new();

    assert!(!mgr.check_access_editable(SHEET_A, ALICE));
    assert!(!mgr.check_access_read_only (SHEET_A, ALICE));
}

#[test]
fn multiple_users_on_same_sheet_hold_independent_rights() {
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::Editable);
    mgr.set_access_right(SHEET_A, BOB,   AccessRight::ReadOnly);

    assert!(mgr.check_access_editable(SHEET_A, ALICE));
    assert!(mgr.check_access_read_only (SHEET_A, BOB));
}

#[test]
fn same_user_can_have_different_rights_on_different_sheets() {
    let mut mgr = AccessControlManager::new();
    mgr.set_access_right(SHEET_A, ALICE, AccessRight::ReadOnly);
    mgr.set_access_right(SHEET_B, ALICE, AccessRight::Editable);

    assert!(mgr.check_access_read_only (SHEET_A, ALICE));
    assert!(mgr.check_access_editable (SHEET_B, ALICE));
}

/// Unsharing a non‑existent user must be a no‑op.
#[test]
fn unshare_nonexistent_user() {
    let mut m = AccessControlManager::new();
    m.set_access_right(SHEET_A, BOB, AccessRight::Editable);

    m.unshared_sheet(SHEET_A, ALICE); // ALICE had no right

    assert!(m.check_access_editable(SHEET_A, BOB));
    assert!(!m.check_access_editable(SHEET_A, ALICE));
}
#[test]
fn remove_nonexistent_sheet() {
    let mut m = AccessControlManager::new();
    m.set_access_right(SHEET_B, ALICE, AccessRight::ReadOnly);

    m.remove_sheet(SHEET_A); // SHEET_A never existed

    assert!(m.check_access_read_only(SHEET_B, ALICE));
}