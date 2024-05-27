use std::io::{self, Write};
use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;

mod user_management;
mod sheet_management;
mod access_control;

pub fn display_menu() {
    println!("---------------Menu---------------");
    println!("1. Create a user");
    println!("2. Create a sheet");
    println!("3. Check a sheet");
    println!("4. Change a value in a sheet");
    println!("5. Change a sheet's access right");
    println!("6. Collaborate with another user");
    println!("7. Unshare a sheet with a user");
    println!("----------------------------------");
    print!("> ");
    io::stdout().flush().unwrap();
}

pub fn handle_input(
    user_manager: &mut UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<u32>().unwrap_or(0);

    match choice {
        1 => user_management::create_user(user_manager),
        2 => sheet_management::create_sheet(user_manager, sheet_manager, access_control_manager),
        3 => sheet_management::check_sheet(user_manager, sheet_manager,access_control_manager),
        4 => sheet_management::change_sheet_value(user_manager, sheet_manager, access_control_manager),
        5 => access_control::change_access_right(user_manager, sheet_manager, access_control_manager),
        6 => access_control::collaborate(user_manager, sheet_manager, access_control_manager),
        7 => access_control::unshare_sheet(user_manager, sheet_manager, access_control_manager),
        _ => println!("Invalid option"),
    }
}