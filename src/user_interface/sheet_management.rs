use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;
use crate::access_control::AccessRight;
use std::io;

/*
    Create a sheet for the user
    If the user does not exist, return an error message
*/
pub fn create_sheet(
    user_manager: &mut UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter username and sheet name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    create_sheet_with_input(input.trim(), user_manager, sheet_manager, access_control_manager);
}

pub fn create_sheet_with_input(
    input: &str,
    user_manager: &mut UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    if let Some(user_id) = user_manager.get_user_id(username) {
        sheet_manager.create_sheet(user_id, sheet_name);
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            access_control_manager.set_access_right(sheet_id, user_id, AccessRight::Editable);
        }
        println!("Create a sheet named \"{}\" for \"{}\".", sheet_name, username);
    } else {
        println!("User not found");
    }
}

/*
    Check the content of the sheet
    If the user has read-only or editable access, they can view the content
*/
pub fn check_sheet(user_manager: &UserManager, sheet_manager: &SheetManager, access_control_manager: &AccessControlManager) {
    println!("Enter username and sheet name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    check_sheet_with_input(input.trim(), user_manager, sheet_manager, access_control_manager);
}

pub fn check_sheet_with_input(input: &str, user_manager: &UserManager, sheet_manager: &SheetManager, access_control_manager: &AccessControlManager) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];

    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            if access_control_manager.check_access_read_only(sheet_id, user_id) || access_control_manager.check_access_editable(sheet_id, user_id) {
                sheet_manager.print_sheet(sheet_id);
            } else {
                println!("You don't have permission to view this sheet.");
            }
        } else {
            println!("Sheet not found.");
        }
    } else {
        println!("User not found.");
    }
}

/*
    Change the value of a cell in the sheet
    If the user has editable access, they can modify the value
*/
pub fn change_sheet_value(user_manager: &UserManager, sheet_manager: &mut SheetManager, access_control_manager: &AccessControlManager) {
    println!("Enter username, sheet name, and cell coordinates with new value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    change_sheet_value_with_input(input.trim(), user_manager, sheet_manager, access_control_manager);
}

pub fn change_sheet_value_with_input(input: &str, user_manager: &UserManager, sheet_manager: &mut SheetManager, access_control_manager: &AccessControlManager) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 4 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    let row = match parts[2].parse::<usize>() {
        Ok(n) if n > 0 => n - 1,
        _ => {
            println!("Invalid row number.");
            return;
        }
    };
    let col = match parts[3].parse::<usize>() {
        Ok(n) if n > 0 => n - 1,
        _ => {
            println!("Invalid column number.");
            return;
        }
    };
    let value = parts[4..].join(" ");

    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            if access_control_manager.check_access_editable(sheet_id, user_id) {
                if let Some(sheet) = sheet_manager.get_sheet_mut(sheet_id) {
                    if row >= sheet.content.len() {
                        sheet.content.resize(row + 1, vec![0.0; sheet.content[0].len()]);
                    }
                    if col >= sheet.content[row].len() {
                        sheet.content[row].resize(col + 1, 0.0);
                    }
                    match crate::arithmetic::evaluate_expression(&value) {
                        Ok(result) => sheet.content[row][col] = result as f32,
                        Err(e) => println!("Error evaluating expression: {}", e),
                    }
                } else {
                    println!("Sheet not found.");
                }
            } else {
                println!("You don't have permission to modify this sheet.");
            }
        } else {
            println!("Sheet not found.");
        }
    } else {
        println!("User not found.");
    }
}

pub fn delete_sheet(
    user_manager: &UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter username and sheet name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    delete_sheet_with_input(input.trim(), user_manager, sheet_manager, access_control_manager);
}

pub fn delete_sheet_with_input(
    input: &str,
    user_manager: &UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];

    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            if sheet_manager.is_owner(user_id, sheet_id) {
                sheet_manager.delete_sheet(sheet_id);
                access_control_manager.remove_sheet(sheet_id);
                println!("Sheet \"{}\" has been deleted.", sheet_name);
            } else {
                println!("You don't have permission to delete this sheet. Only the owner can delete it.");
            }
        } else {
            println!("Sheet not found.");
        }
    } else {
        println!("User not found.");
    }
}