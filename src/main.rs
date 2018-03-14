#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![no_main]

extern crate rlibc;

static HELLO: &[u8] = b"Hello World";

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
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color: ColorCode,
}

const BUFFER_WIDTH: usize = 25;
const BUFFER_HEIGHT: usize = 80;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
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

                self.buffer.chars[row][col] = ScreenChar {
                    character: byte,
                    color: self.color_code,
                };
                self.column_position += 1;
            }
        } 
    }
    
    fn clear_line(&mut self, line: usize) {
        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[line][x] = ScreenChar {
                character: 0x20,
                color: ColorCode::new(Color::White, Color::Black),
            };
        }
    }

    fn new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH-1 {   
                self.buffer.chars[y-1][x] = self.buffer.chars[y][x];
            }
        }
    
        self.clear_line(BUFFER_HEIGHT-1);
        self.column_position = 0; 
    }
}

#[no_mangle]
pub extern fn _start() -> ! {
    
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, 
    };

    loop {
        writer.write_str("Hello World\n");
    }

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> ! {
    loop {}
}
