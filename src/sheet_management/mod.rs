use std::sync::Mutex;

// Define the Sheet struct
pub struct Sheet {
    id: u32,
    owner_id: u32,
    sheet_name: String,
    pub content: Vec<Vec<f32>> // use (u32, u32) coordinates as the key to represent the cell
}

// Create a struct to manage the sheets
pub struct SheetManager {
    sheets: Vec<Sheet>,
    next_id: Mutex<u32>,
}

impl SheetManager {
    pub fn new() -> Self {
        Self {
            sheets: Vec::new(),
            next_id: Mutex::new(1),
        }
    }
    
    // Create a new sheet
    pub fn create_sheet(&mut self, owner_id: u32, sheet_name: &str) {
        let mut id_guard = self.next_id.lock().unwrap();
        let sheet = Sheet {
            id: *id_guard,
            owner_id,
            sheet_name: sheet_name.to_string(),
            content: vec![vec![0.0; 3]; 3],
        };
        
        *id_guard += 1;
        self.sheets.push(sheet);
    }
    
    // Get the sheet by ID
    fn get_sheet(&self, sheet_id: u32) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.id == sheet_id)
    }
    
    // Get the sheet by ID (mutable)
    pub fn get_sheet_mut(&mut self, sheet_id: u32) -> Option<&mut Sheet> {
        self.sheets.iter_mut().find(|sheet| sheet.id == sheet_id)
    }
    
    // Get the sheet ID by name
    pub fn get_sheet_id(&self, sheet_name: &str) -> Option<u32> {
        if let Some(sheet) = self.sheets.iter().find(|sheet| sheet.sheet_name == sheet_name) {
            Some(sheet.id)
        } else {
            None
        }
    }
    
    // Print the content of the sheet
    pub fn print_sheet(&self, sheet_id: u32) {
        if let Some(sheet) = self.get_sheet(sheet_id) {
            // Print the sheet information
            println!("Sheet Name: {}", sheet.sheet_name);
            println!("Content:");
            
            // Print the content of the sheet
            for row in &sheet.content {
                for cell in row {
                    print!("{} ", cell);
                }
                println!();
            }

        } else { // If the sheet does not exist
            println!("Sheet not found!");
        }
    }
    
    pub fn is_owner(&self, user_id: u32, sheet_id: u32) -> bool {
        self.sheets
            .iter()
            .find(|sheet| sheet.id == sheet_id)
            .map_or(false, |sheet| sheet.owner_id == user_id)
    }
    
    // Delete a sheet
    pub fn delete_sheet(&mut self, sheet_id: u32) {
        if let Some(pos) = self.sheets.iter().position(|sheet| sheet.id == sheet_id) {
            self.sheets.remove(pos);
        } else {
            println!("Sheet not found!");
        }
    }
}

#[cfg(test)]
mod tests;