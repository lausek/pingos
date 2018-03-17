extern crate volatile;

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

pub struct Buffer<T: 'static + Copy> {
    pub chars: &'static mut [[Volatile<T>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    pub height: usize,
    pub width: usize,
}

impl <T> Buffer<T>
    where T: Copy
{
    
    pub fn new() -> Buffer<T> {
        Buffer {
            chars: unsafe { &mut *(0xb8000 as *mut [[Volatile<T>; BUFFER_WIDTH]; BUFFER_HEIGHT]) },
            height: BUFFER_HEIGHT,
            width: BUFFER_WIDTH,
        } 
    }

}
