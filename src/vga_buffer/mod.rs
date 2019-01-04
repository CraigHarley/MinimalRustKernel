use lazy_static::lazy_static;

use crate::vga_buffer::color::Color;
use crate::vga_buffer::writer::Writer;
use crate::vga_buffer::color_code::ColorCode;
use crate::vga_buffer::buffer::Buffer;

mod writer;
mod color;
mod color_code;
mod screen_char;
mod buffer;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static WRITER: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
}