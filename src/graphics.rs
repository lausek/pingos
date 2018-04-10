extern crate volatile;

use vga::ScreenChar;
use core::ptr::Unique;
use self::volatile::Volatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 13,
    Yellow      = 14,
    White       = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub type VgaBuffer = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub struct Buffer {
    pub chars: Unique<VgaBuffer>, //&'static mut [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    pub height: usize,
    pub width: usize,
}

impl Buffer {
    
    pub const fn new() -> Buffer {
        Buffer {
            chars: unsafe { Unique::new_unchecked(0xb8000 as *mut _) }, //unsafe { &mut *(0xb8000 as *mut [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]) },
            height: BUFFER_HEIGHT,
            width: BUFFER_WIDTH,
        } 
    }

}
