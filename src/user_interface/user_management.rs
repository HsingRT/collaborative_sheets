use crate::user_management::UserManager;
use std::io;

pub fn create_user(user_manager: &mut UserManager) {
    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    user_manager.create_user(username);
    println!("Create a user named \"{}\".", username);
}