use std::io::{self, Write};
use crate::user_management::UserManager;
use crate::sheet_management::SheetManager;
use crate::access_control::AccessControlManager;

mod user_management;
mod sheet_management;
mod access_control;


// Display the menu
pub fn display_menu() {
    println!("---------------Menu---------------");
    println!("1. Create a user");
    println!("2. Create a sheet");
    println!("3. Check a sheet");
    println!("4. Change a value in a sheet");
    println!("5. Delete a sheet");
    println!("6. Change a sheet's access right");
    println!("7. Collaborate with another user");
    println!("8. Unshared a sheet with a user");
    println!("9. Delete a user");
    println!("10. Exit");
    println!("----------------------------------");
    print!("> ");
    io::stdout().flush().unwrap();
}

// Handle the user input
pub fn handle_input(
    user_manager: &mut UserManager,
    sheet_manager: &mut SheetManager,
    access_control_manager: &mut AccessControlManager,
) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<u32>().unwrap_or(0);

    // Handle the user's choice
    match choice {
        1 => user_management::create_user(user_manager),
        2 => sheet_management::create_sheet(user_manager, sheet_manager, access_control_manager),
        3 => sheet_management::check_sheet(user_manager, sheet_manager,access_control_manager),
        4 => sheet_management::change_sheet_value(user_manager, sheet_manager, access_control_manager),
        5 => sheet_management::delete_sheet(user_manager, sheet_manager, access_control_manager),
        6 => access_control::change_access_right(user_manager, sheet_manager, access_control_manager),
        7 => access_control::collaborate(user_manager, sheet_manager, access_control_manager),
        8 => access_control::unshared_sheet(user_manager, sheet_manager, access_control_manager),
        9 => user_management::delete_user(user_manager),
        10 => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => println!("Invalid option"),
    }
}

#[cfg(test)]
mod tests;