use super::*;

/// IDs must be unique and monotonically increasing
#[test]
fn test_multiple_sheet_ids_are_unique_and_monotonic() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(1, "A");
    mgr.create_sheet(2, "B");

    let id_a = mgr.get_sheet_id("A").unwrap();
    let id_b = mgr.get_sheet_id("B").unwrap();

    assert_ne!(id_a, id_b);
    assert!(id_b > id_a);
}

/// Every newly‑created sheet starts with a 3 × 3 zero grid
#[test]
fn test_new_sheet_has_zeroed_content() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(7, "ZeroGrid");

    let id = mgr.get_sheet_id("ZeroGrid").unwrap();
    let sheet = mgr.get_sheet(id).unwrap();

    assert!(sheet
        .content
        .iter()
        .all(|row| row.iter().all(|&c| c == 0.0)));
}

/// Editing one sheet must not leak into another
#[test]
fn test_sheet_mutation_is_isolated() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(1, "S1");
    mgr.create_sheet(2, "S2");

    let id1 = mgr.get_sheet_id("S1").unwrap();
    let id2 = mgr.get_sheet_id("S2").unwrap();

    mgr.get_sheet_mut(id1).unwrap().content[0][0] = 5.0;

    assert_eq!(mgr.get_sheet(id1).unwrap().content[0][0], 5.0);
    assert_eq!(mgr.get_sheet(id2).unwrap().content[0][0], 0.0);
}

/// get_sheet / get_sheet_mut must return None for unknown ID
#[test]
fn test_get_sheet_returns_none_for_missing_id() {
    let mut mgr = SheetManager::new();
    assert!(mgr.get_sheet(123).is_none());
    assert!(mgr.get_sheet_mut(456).is_none());
}

/// is_owner should behave correctly for all code paths
#[test]
fn test_is_owner_edge_cases() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(9, "Owned");
    let id = mgr.get_sheet_id("Owned").unwrap();

    assert!(mgr.is_owner(9, id));       // true for owner
    assert!(!mgr.is_owner(8, id));      // false for non‑owner
    assert!(!mgr.is_owner(9, 9999));    // false for missing sheet
}

/// Deleting a sheet removes it and remains safe if repeated
#[test]
fn test_delete_sheet_semantics() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(1, "Temp");
    let id = mgr.get_sheet_id("Temp").unwrap();

    mgr.delete_sheet(id);
    assert!(mgr.get_sheet_id("Temp").is_none());

    // second delete must be a no‑op
    mgr.delete_sheet(id);
}

/// ID counter must keep advancing after deletion
#[test]
fn test_id_counter_continues_after_deletion() {
    let mut mgr = SheetManager::new();
    mgr.create_sheet(1, "First");
    let first_id = mgr.get_sheet_id("First").unwrap();

    mgr.delete_sheet(first_id);

    mgr.create_sheet(2, "Second");
    let second_id = mgr.get_sheet_id("Second").unwrap();

    assert!(second_id > first_id);
}
