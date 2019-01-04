mod writer;
mod color;
mod color_code;
mod screen_char;
mod buffer;

use crate::vga_buffer::color::Color;
use crate::vga_buffer::writer::Writer;
use crate::vga_buffer::color_code::ColorCode;
use crate::vga_buffer::buffer::Buffer;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(
            Color::Yellow,
            Color::Black,
        ),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
