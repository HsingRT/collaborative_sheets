#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_access_right() {
        let mut manager = AccessControlManager::new();
        manager.set_access_right(1, 42, AccessRight::Editable);

        assert!(manager.check_access_editable(1, 42));
        assert!(!manager.check_access_read_only(1, 42));
    }

    #[test]
    fn test_override_existing_access_right() {
        let mut manager = AccessControlManager::new();
        manager.set_access_right(1, 42, AccessRight::ReadOnly);
        assert!(manager.check_access_read_only(1, 42));

        // Update to Editable
        manager.set_access_right(1, 42, AccessRight::Editable);
        assert!(manager.check_access_editable(1, 42));
        assert!(!manager.check_access_read_only(1, 42));
    }

    #[test]
    fn test_share_sheet() {
        let mut manager = AccessControlManager::new();
        manager.share_sheet(1, 100, AccessRight::ReadOnly);

        assert!(manager.check_access_read_only(1, 100));
    }

    #[test]
    fn test_unshare_sheet() {
        let mut manager = AccessControlManager::new();
        manager.set_access_right(1, 200, AccessRight::Editable);

        assert!(manager.check_access_editable(1, 200));

        manager.unshared_sheet(1, 200);
        assert!(!manager.check_access_editable(1, 200));
        assert!(!manager.check_access_read_only(1, 200));
    }

    #[test]
    fn test_multiple_users_on_same_sheet() {
        let mut manager = AccessControlManager::new();
        manager.set_access_right(1, 1, AccessRight::Editable);
        manager.set_access_right(1, 2, AccessRight::ReadOnly);

        assert!(manager.check_access_editable(1, 1));
        assert!(manager.check_access_read_only(1, 2));
        assert!(!manager.check_access_editable(1, 2));
    }

    #[test]
    fn test_access_right_on_nonexistent_sheet_or_user() {
        let manager = AccessControlManager::new();

        assert!(!manager.check_access_editable(99, 42));
        assert!(!manager.check_access_read_only(99, 42));
    }
}