use super::*;
use crate::vga_buffer::screen_char::ScreenChar;

fn empty_char() -> ScreenChar {
    ScreenChar {
        ascii_character: b' ',
        color_code: ColorCode::new(Color::Green, Color::Brown),
    }
}

fn construct_writer() -> Writer {
    use std::boxed::Box;

    let buffer = construct_buffer();
    Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::Magenta),
        buffer: Box::leak(Box::new(buffer)),
    }
}

fn construct_buffer() -> Buffer {
    use array_init::array_init;
    use volatile::Volatile;

    Buffer {
        chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
    }
}

#[test]
fn write_byte() {
    let mut writer = construct_writer();
    writer.write_byte(b'X');
    writer.write_byte(b'Y');

    for (i, row) in writer.buffer.chars.iter().enumerate() {
        for (j, screen_char) in row.iter().enumerate() {
            let screen_char = screen_char.read();
            if i == BUFFER_HEIGHT - 1 && j == 0 {
                assert_eq!(screen_char.ascii_character, b'X');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                assert_eq!(screen_char.ascii_character, b'Y');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else {
                assert_eq!(screen_char, empty_char());
            }
        }
    }
}

#[test]
fn write_formatted() {
    use core::fmt::Write;

    let mut writer = construct_writer();
    writeln!(&mut writer, "a").unwrap();
    writeln!(&mut writer, "b{}", "c").unwrap();

    for (i, row) in writer.buffer.chars.iter().enumerate() {
        for (j, screen_char) in row.iter().enumerate() {
            let screen_char = screen_char.read();
            if i == BUFFER_HEIGHT - 3 && j == 0 {
                assert_eq!(screen_char.ascii_character, b'a');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                assert_eq!(screen_char.ascii_character, b'b');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                assert_eq!(screen_char.ascii_character, b'c');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else if i >= BUFFER_HEIGHT - 2 {
                assert_eq!(screen_char.ascii_character, b' ');
                assert_eq!(screen_char.color_code, writer.color_code);
            } else {
                assert_eq!(screen_char, empty_char());
            }
        }
    }
}
