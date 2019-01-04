use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

use crate::vga_buffer::writer::Writer;

mod writer;
mod color;
mod color_code;
mod screen_char;
mod buffer;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
//    WRITER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static WRITER: Mutex<Writer> = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
}
