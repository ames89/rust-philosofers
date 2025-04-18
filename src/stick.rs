use std::sync::Mutex; 

pub struct Stick {
    pub status: Mutex<()>,
}