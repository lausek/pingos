use vga::Writer;

use core::fmt::Write;

macro_rules! log {
    ($l:ident, $msg:expr) => ($l.lock().add($msg)); 
    ($l:ident, $msg:expr, $($param:expr),*) => ($l.lock().add(format!($msg, $($param),*))); 
}

pub struct Logger {
    destination: Option<u32>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            destination: None,
        }
    }

    pub fn add(&mut self, msg: &str) {
        vga_write!(msg);
    }
}
