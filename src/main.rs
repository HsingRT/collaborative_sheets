mod user_management;
mod sheet_management;
mod access_control;
mod arithmetic;
mod user_interface;

use user_management::UserManager;
use sheet_management::SheetManager;
use access_control::AccessControlManager;

fn main() {
    let mut user_manager = UserManager::new();
    let mut sheet_manager = SheetManager::new();
    let mut access_control_manager = AccessControlManager::new();

    loop {
        user_interface::display_menu();
        user_interface::handle_input(&mut user_manager, &mut sheet_manager, &mut access_control_manager);
    }
}