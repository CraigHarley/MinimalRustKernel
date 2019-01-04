use volatile::Volatile;
use crate::vga_buffer::screen_char::ScreenChar;
use crate::vga_buffer::BUFFER_WIDTH;
use crate::vga_buffer::BUFFER_HEIGHT;

pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
