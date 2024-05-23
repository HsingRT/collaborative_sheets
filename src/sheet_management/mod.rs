use std::sync::Mutex;

struct Sheet {
    id: u32,
    owner_id: u32,
    sheet_name: String,
    content: Vec<Vec<f32>> // use (u32, u32) coordinates as the key to represent the cell
}

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
    
    pub fn create_sheet(&mut self, owner_id: u32, sheet_name: &str) {
        let mut id_guard = self.next_id.lock().unwrap();
        let sheet = Sheet {
            id: *id_guard,
            owner_id,
            sheet_name: sheet_name.to_string(),
            content: Vec::new(),
        };
        
        *id_guard += 1;
        self.sheets.push(sheet);
    }
    
    fn get_sheet(&self, sheet_id: u32) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.id == sheet_id)
    }
    
    fn get_sheet_mut(&mut self, sheet_id: u32) -> Option<&mut Sheet> {
        self.sheets.iter_mut().find(|sheet| sheet.id == sheet_id)
    }
    
    pub fn get_sheet_id(&self, sheet_name: &str) -> Option<u32> {
        if let Some(sheet) = self.sheets.iter().find(|sheet| sheet.sheet_name == sheet_name) {
            Some(sheet.id)
        } else {
            None
        }
    }
    
    pub fn delete_sheet(&mut self, sheet_id: u32) -> bool {
        if let Some(index) = self.sheets.iter().position(|sheet| sheet.id == sheet_id) {
            self.sheets.remove(index);
            true
        } else {
            false
        }
    }
    
    pub fn print_sheet(&self, sheet_id: u32) {
        if let Some(sheet) = self.get_sheet(sheet_id) {
            // Print the sheet information
            println!("Sheet ID: {}", sheet.id);
            println!("Sheet Name: {}", sheet.sheet_name);
            println!("Owner ID: {}", sheet.owner_id);
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
    
}  