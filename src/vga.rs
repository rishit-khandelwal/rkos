use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

struct Charecter {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Charecter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGAWriter {
    pub cpos: usize, // Position horizontally,
    pub rpos: usize, // Position vertically,
    pub buffer: &'static mut Buffer,
}

impl VGAWriter {
    pub fn write(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.rpos += 1;
                self.cpos = 0;
            }
            byte => {
                if self.cpos >= BUFFER_WIDTH {
                    self.rpos += 1;
                    self.cpos = 0;
                }

                let row = self.rpos;
                let col = self.cpos;

                let color_code = ColorCode::new(Color::Green, Color::Black);
                self.buffer.chars[row][col] = Charecter {
                    ascii_char: byte,
                    color_code,
                };
                self.cpos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        s.chars().into_iter().for_each(|c| {
            self.write(c as u8);
        });
    }
}

impl fmt::Write for VGAWriter {
    fn write_fmt(mut self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.write_string(args.as_str().unwrap());
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($writer:ident, $($args:expr),*) => {
        fmt::write(&mut $writer, format_args!($($args),*)).unwrap();
    };
}
