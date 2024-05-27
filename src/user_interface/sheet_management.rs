use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;
use crate::access_control::AccessRight;
use std::io;

pub fn create_sheet(
    user_manager: &mut UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    println!("Enter username and sheet name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    if let Some(user_id) = user_manager.get_user_id(username) {
        sheet_manager.create_sheet(user_id as u32, sheet_name);
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            access_control_manager.set_access_right(sheet_id, user_id as u32, AccessRight::Editable);
        }
        println!("Create a sheet named \"{}\" for \"{}\".", sheet_name, username);
    } else {
        println!("User not found");
    }
}

pub fn check_sheet(user_manager: &UserManager, sheet_manager: &SheetManager) {
    println!("Enter username and sheet name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            sheet_manager.print_sheet(sheet_id);
        } else {
            println!("Sheet not found");
        }
    } else {
        println!("User not found");
    }
}

pub fn change_sheet_value(user_manager: &UserManager, sheet_manager: &mut SheetManager) {
    println!("Enter username, sheet name, and cell coordinates with new value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 4 {
        println!("Invalid input");
        return;
    }
    let username = parts[0];
    let sheet_name = parts[1];
    let row = parts[2].parse::<usize>().unwrap_or(0);
    let col = parts[3].parse::<usize>().unwrap_or(0);
    let value = parts[4..].join(" ");
    if let Some(user_id) = user_manager.get_user_id(username) {
        if let Some(sheet_id) = sheet_manager.get_sheet_id(sheet_name) {
            if let Some(sheet) = sheet_manager.get_sheet_mut(sheet_id) {
                if row >= sheet.content.len() {
                    sheet.content.resize(row + 1, vec![]);
                }
                if col >= sheet.content[row].len() {
                    sheet.content[row].resize(col + 1, 0.0);
                }
                match crate::arithmetic::evaluate_expression(&value) {
                    Ok(result) => sheet.content[row][col] = result as f32,
                    Err(e) => println!("Error evaluating expression: {}", e),
                }
            } else {
                println!("Sheet not found");
            }
        } else {
            println!("Sheet not found");
        }
    } else {
        println!("User not found");
    }
}