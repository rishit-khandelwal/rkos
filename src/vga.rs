use core::fmt::{self, Write};

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
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub struct Charecter {
    pub ascii_char: u8,
    pub color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Charecter; BUFFER_WIDTH]; BUFFER_HEIGHT],
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

impl Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter {
        cpos: 0,
        rpos: 0,

        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
