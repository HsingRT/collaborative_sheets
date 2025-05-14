use super::*;

#[test]
fn test_create_sheet_and_get_id() {
    let mut manager = SheetManager::new();
    manager.create_sheet(1, "Budget");

    let sheet_id = manager.get_sheet_id("Budget");
    assert!(sheet_id.is_some());
}

#[test]
fn test_get_sheet_mut_and_edit_content() {
    let mut manager = SheetManager::new();
    manager.create_sheet(1, "Grades");

    let sheet_id = manager.get_sheet_id("Grades").unwrap();
    let sheet = manager.get_sheet_mut(sheet_id).unwrap();
    sheet.content[0][0] = 99.9;

    assert_eq!(sheet.content[0][0], 99.9);
}

#[test]
fn test_is_owner_true() {
    let mut manager = SheetManager::new();
    manager.create_sheet(42, "Project");

    let sheet_id = manager.get_sheet_id("Project").unwrap();
    assert!(manager.is_owner(42, sheet_id));
}

#[test]
fn test_is_owner_false() {
    let mut manager = SheetManager::new();
    manager.create_sheet(1, "Team Plan");

    let sheet_id = manager.get_sheet_id("Team Plan").unwrap();
    assert!(!manager.is_owner(999, sheet_id));
}

#[test]
fn test_delete_sheet() {
    let mut manager = SheetManager::new();
    manager.create_sheet(1, "Temp");

    let sheet_id = manager.get_sheet_id("Temp").unwrap();
    manager.delete_sheet(sheet_id);

    assert!(manager.get_sheet_id("Temp").is_none());
}