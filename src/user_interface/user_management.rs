use crate::user_management::UserManager;
use std::io;

// Create a user
pub fn create_user(user_manager: &mut UserManager) {
    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    create_user_with_name(username, user_manager);
}

pub fn create_user_with_name(username: &str, user_manager: &mut UserManager) {
    if user_manager.user_exists(username) {
        println!("User \"{}\" already exists.", username);
        return;
    }

    user_manager.create_user(username);
    println!("Create a user named \"{}\".", username);
}

pub fn delete_user(user_manager: &mut UserManager) {
    println!("Enter username to delete:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    delete_user_with_name(username, user_manager);
}

pub fn delete_user_with_name(username: &str, user_manager: &mut UserManager) {
    if user_manager.delete_user(username) {
        println!("Deleted user \"{}\".", username);
    } else {
        println!("User \"{}\" not found.", username);
    }
}