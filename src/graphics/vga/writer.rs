extern crate volatile;

use core::{fmt, ptr::Unique};
use spin::Mutex;
use graphics::color::{Color, ColorCode};
use self::volatile::Volatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub type VgaBuffer = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new()); 
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    character: u8,
    color: ColorCode,
}

impl ScreenChar {
    pub const fn new(character: u8, color: ColorCode) -> ScreenChar {
        ScreenChar {
            character,
            color,
        }
    }
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    chars: Unique<VgaBuffer>,
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Green, Color::Black),
            chars: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
        }
    }
   
    fn chars(&mut self) -> &mut VgaBuffer {
        unsafe {
            self.chars.as_mut()
        }
    }

    pub fn write_str(&mut self, content: &str) {
        for byte in content.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if BUFFER_WIDTH <= self.column_position {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color = self.color_code;

                self.chars()[row][col].write(ScreenChar {
                    character: byte,
                    color: color,
                });
                self.column_position += 1;
            }
        } 
    }
    
    fn clear_line(&mut self, line: usize) {
        let color = ColorCode::new(Color::White, Color::Black);
        for x in 0..BUFFER_WIDTH {
            self.chars()[line][x].write(ScreenChar::new(0x20, color));
        }
    }

    fn new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH-1 {   
                let c = self.chars()[y][x].read();
                self.chars()[y-1][x].write(c);
            }
        }
   
        let lines = BUFFER_HEIGHT - 1;
        self.clear_line(lines);
        self.column_position = 0; 
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}
