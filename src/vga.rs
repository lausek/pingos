extern crate volatile;

use core::{fmt, fmt::Write};
use spin::Mutex;
use graphics::{Buffer, Color, ColorCode, VgaBuffer};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new()); 
}

macro_rules! vga_write {
    ($str:expr) => (write!(::vga::WRITER.lock(), "{}", $str).ok());
    ($str:expr, $($args:ident),*) => (write!(::vga::WRITER.lock(), "{}", $str, $($args),*).ok());
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
    buffer: Buffer,
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Green, Color::Black),
            buffer: Buffer::new(),
        }
    }
    
    fn buffer(&mut self) -> &mut VgaBuffer {
        unsafe { self.buffer.chars.as_mut() }
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
                if self.buffer.width <= self.column_position {
                    self.new_line();
                }

                let row = self.buffer.height - 1;
                let col = self.column_position;
                let color = self.color_code;

                self.buffer()[row][col].write(ScreenChar {
                    character: byte,
                    color: color,
                });
                self.column_position += 1;
            }
        } 
    }
    
    fn clear_line(&mut self, line: usize) {
        let color = ColorCode::new(Color::White, Color::Black);
        for x in 0..self.buffer.width {
            self.buffer()[line][x].write(ScreenChar::new(0x20, color));
        }
    }

    fn new_line(&mut self) {
        for y in 1..self.buffer.height {
            for x in 0..self.buffer.width-1 {   
                let c = self.buffer()[y][x].read();
                self.buffer()[y-1][x].write(c);
            }
        }
   
        let lines = self.buffer.height.clone() - 1;
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
