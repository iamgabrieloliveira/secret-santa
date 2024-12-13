use std::collections::HashMap;
use std::sync::Mutex;

pub struct AppState {
    pub participants: Mutex<Vec<String>>,
    pub links: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            participants: Mutex::new(Vec::new()),
            links: Mutex::new(HashMap::new()),
        }
    }
}
