use std::collections::HashMap;

#[derive(Clone, Copy)]

/*
* Enum to represent the access right of the user to the sheet
*/
pub
enum AccessRight {
    ReadOnly,
    Editable,
}

struct AccessControl {
    user_id: u32,
    access_right: AccessRight,
}

struct AccessControlManager {
    access_controls: HashMap<u32, Vec<AccessControl>>, // use sheet_id as the key to represent the sheet's access control
}

impl AccessControlManager {
    pub fn new() -> Self {
        Self {
            access_controls: HashMap::new(),
        }
    }
    
    /* 
     * Set the access right of the user to the sheet
     * If the sheet does not exist, create a new sheet and set the access right
     */
    fn set_access_right(&mut self, sheet_id: u32, user_id: u32, access_right: AccessRight) {
        let access_control = AccessControl {
            user_id,
            access_right,
        };
        
        if let Some(access_controls) = self.access_controls.get_mut(&sheet_id) {
            if let Some(access_control) = access_controls.iter_mut().find(|access_control| access_control.user_id == user_id) {
                access_control.access_right = access_right;
            } else {
                access_controls.push(access_control);
            }
        } else {
            self.access_controls.insert(sheet_id, vec![access_control]);
        }
    }
    
    /* 
     * Get the access right of the user to the sheet
     */
    fn get_access_right(&self, sheet_id: u32, user_id: u32) -> Option<&AccessControl> {
        if let Some(access_controls) = self.access_controls.get(&sheet_id) {
            access_controls.iter().find(|access_control| access_control.user_id == user_id)
        } else {
            None
        }
    }
    
    /* 
     * Update the access right of the user to the sheet
     */
    pub fn share_sheet(&mut self, sheet_id: u32, to_user_id: u32, access_right: AccessRight) {
        self.set_access_right(sheet_id, to_user_id, access_right);
    }
    
    /* 
     * Remove the access right of the user to the sheet
     */
    pub fn unshared_sheet(&mut self, sheet_id: u32, user_id: u32) {
        if let Some(access_controls) = self.access_controls.get_mut(&sheet_id) {
            access_controls.retain(|access_control| access_control.user_id != user_id); // Remove the access control of the user
        }
    }
    
    /* 
     * Check the access right of the user to the sheet
     */
    pub fn check_access_right(&self, sheet_id: u32, user_id: u32) -> bool {
        if let Some(access_control) = self.get_access_right(sheet_id, user_id) {
            access_control.user_id == user_id
        } else {
            false
        }
    }
    
}