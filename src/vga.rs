use graphics;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    character: u8,
    color: graphics::ColorCode,
}

pub struct Writer {
    column_position: usize,
    color_code: graphics::ColorCode,
    buffer: graphics::Buffer<'static, ScreenChar>,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            column_position: 0,
            color_code: graphics::ColorCode::new(graphics::Color::White, graphics::Color::Black),
            buffer: graphics::Buffer::<ScreenChar>::new(),
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
                if self.buffer.width <= self.column_position {
                    self.new_line();
                }

                let row = self.buffer.height - 1;
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
        for x in 0..self.buffer.width {
            self.buffer.chars[line][x] = ScreenChar {
                character: 0x20,
                color: graphics::ColorCode::new(graphics::Color::White, graphics::Color::Black),
            };
        }
    }

    fn new_line(&mut self) {
        for y in 1..self.buffer.height {
            for x in 0..self.buffer.width-1 {   
                self.buffer.chars[y-1][x] = self.buffer.chars[y][x];
            }
        }
   
        let lines = self.buffer.height.clone() - 1;
        self.clear_line(lines);
        self.column_position = 0; 
    }
}
