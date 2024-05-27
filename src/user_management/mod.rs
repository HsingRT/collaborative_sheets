use std::sync::Mutex;

struct User {
    id : u32,
    username : String,
}

pub struct UserManager {
    users: Vec<User>,
    next_id: Mutex<u32>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            next_id: Mutex::new(1),
        }
    }

    pub fn create_user(&mut self, username: &str) {
        let mut id_guard = self.next_id.lock().unwrap();
        let user = User {
            id: *id_guard,
            username: username.to_string(),
        };

        *id_guard += 1; // Update the id counter
        self.users.push(user);
    }
    
    fn get_user(&self, username: &str) -> Option<&User> {
        if let Some(user) = self.users.iter().find(|user| user.username == username) {
            Some(user)
        } else {
            None
        }
    }
    
    pub fn get_user_id(&self, username: &str) -> Option<u32> {
        if let Some(user) = self.get_user(username) {
            Some(user.id as u32)
        } else {
            None
        }
    }
}
