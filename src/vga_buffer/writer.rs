use crate::vga_buffer::color_code::ColorCode;
use crate::vga_buffer::screen_char::ScreenChar;
use crate::vga_buffer::buffer::Buffer;
use crate::vga_buffer::BUFFER_WIDTH;
use crate::vga_buffer::BUFFER_HEIGHT;

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) { /* TODO */ }
}