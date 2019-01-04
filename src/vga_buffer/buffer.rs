use crate::vga_buffer::screen_char::ScreenChar;
use crate::vga_buffer::BUFFER_HEIGHT;
use crate::vga_buffer::BUFFER_WIDTH;
use volatile::Volatile;

pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
