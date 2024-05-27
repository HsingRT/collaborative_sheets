use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::{AccessControlManager, AccessRight};
use std::io;

// pub fn change_access_right(
//     user_manager: &UserManager,
//     sheet_manager: &SheetManager,
//     access_control_manager: &mut AccessControlManager,
// ) {
//     println!("Enter username, sheet name, and new access right (ReadOnly(0)/Editable(1)):");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let parts: Vec<&str> = input.trim().split_whitespace().collect();
//     if parts.len() != 3 {
//         println!("Invalid input");
//         return;
//     }
//     let username = parts[0];
//     let sheet_name = parts[1];
//     let access_right = match parts[2] {
//         "0" => AccessRight::ReadOnly,
//         "1" => AccessRight::Editable,
//         _ => {
//             println!("Invalid access right");
//             return;
//         }
//     };
//     if let Some(user_id) = user_manager.get_user_id(username) {
//         if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
//             access_control_manager.set_access_right(sheet_id, user_id as u32, access_right);
//         } else {
//             println!("Sheet not found");
//         }
//     } else {
//         println!("User not found");
//     }
// }

// pub fn collaborate(
//     user_manager: &UserManager,
//     sheet_manager: &SheetManager,
//     access_control_manager: &mut AccessControlManager,
// ) {
//     println!("Enter owner username, sheet name, and collaborator username:");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let parts: Vec<&str> = input.trim().split_whitespace().collect();
//     if parts.len() != 3 {
//         println!("Invalid input");
//         return;
//     }
//     let owner_username = parts[0];
//     let sheet_name = parts[1];
//     let collaborator_username = parts[2];
//     if let Some(_owner_id) = user_manager.get_user_id(owner_username) {
//         if let Some(collaborator_id) = user_manager.get_user_id(collaborator_username) {
//             if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
//                 access_control_manager.share_sheet(sheet_id, collaborator_id as u32, AccessRight::Editable);
//                 println!("Share \"{}\"'s \"{}\" with \"{}\".", owner_username, sheet_name, collaborator_username);
//             } else {
//                 println!("Sheet not found");
//             }
//         } else {
//             println!("Collaborator not found");
//         }
//     } else {
//         println!("Owner not found");
//     }
// }

// pub fn unshare_sheet(
//     user_manager: &UserManager,
//     sheet_manager: &SheetManager,
//     access_control_manager: &mut AccessControlManager,
// ) {
//     println!("Enter username, sheet name, and collaborator username to unshare:");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let parts: Vec<&str> = input.trim().split_whitespace().collect();
//     if parts.len() != 3 {
//         println!("Invalid input");
//         return;
//     }
//     let owner_username = parts[0];
//     let sheet_name = parts[1];
//     let collaborator_username = parts[2];
//     if let Some(_owner_id) = user_manager.get_user_id(owner_username) {
//         if let Some(collaborator_id) = user_manager.get_user_id(collaborator_username) {
//             if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
//                 access_control_manager.unshared_sheet(sheet_id, collaborator_id as u32);
//                 println!("Unshare \"{}\"'s \"{}\" with \"{}\".", owner_username, sheet_name, collaborator_username);
//             } else {
//                 println!("Sheet not found");
//             }
//         } else {
//             println!("Collaborator not found");
//         }
//     } else {
//         println!("Owner not found");
//     }
// }



pub fn change_access_right(
    user_manager: &UserManager,
    sheet_manager: &SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter username, sheet name, and new access right (ReadOnly(0)/Editable(1)):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    let access_right = match parts[2] {
        "0" => AccessRight::ReadOnly,
        "1" => AccessRight::Editable,
        _ => {
            println!("Invalid access right");
            return;
        }
    };

    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            if access_control_manager.check_access_editable(sheet_id, user_id as u32) {
                access_control_manager.set_access_right(sheet_id, user_id as u32, access_right);
            } else {
                println!("You don't have permission to change access right for this sheet");
            }
        } else {
            println!("Sheet not found");
        }
    } else {
        println!("User not found");
    }
}

pub fn collaborate(
    user_manager: &UserManager,
    sheet_manager: &SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter owner username, sheet name, and collaborator username:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid input");
        return;
    }
    let owner_username = parts[0];
    let sheet_name = parts[1];
    let collaborator_username = parts[2];
    
    if let Some(owner_id) = user_manager.get_user_id(owner_username) {
        if let Some(collaborator_id) = user_manager.get_user_id(collaborator_username) {
            if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
                if access_control_manager.check_access_editable(sheet_id, owner_id as u32) {
                    access_control_manager.share_sheet(sheet_id, collaborator_id as u32, AccessRight::Editable);
                    println!("Share \"{}\"'s \"{}\" with \"{}\".", owner_username, sheet_name, collaborator_username);
                } else {
                    println!("You don't have permission to share this sheet");
                }
            } else {
                println!("Sheet not found");
            }
        } else {
            println!("Collaborator not found");
        }
    } else {
        println!("Owner not found");
    }
}

pub fn unshare_sheet(
    user_manager: &UserManager,
    sheet_manager: &SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter username, sheet name, and collaborator username to unshare:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid input");
        return;
    }
    let owner_username = parts[0];
    let sheet_name = parts[1];
    let collaborator_username = parts[2];
    
    if let Some(owner_id) = user_manager.get_user_id(owner_username) {
        if let Some(collaborator_id) = user_manager.get_user_id(collaborator_username) {
            if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
                if access_control_manager.check_access_editable(sheet_id, owner_id as u32) {
                    access_control_manager.unshared_sheet(sheet_id, collaborator_id as u32);
                    println!("Unshare \"{}\"'s \"{}\" with \"{}\".", owner_username, sheet_name, collaborator_username);
                } else {
                    println!("You don't have permission to unshare this sheet");
                }
            } else {
                println!("Sheet not found");
            }
        } else {
            println!("Collaborator not found");
        }
    } else {
        println!("Owner not found");
    }
}