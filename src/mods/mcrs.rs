use lazy_static::lazy_static;

use super::vga;
use core::fmt::Write;
use spin::Mutex;
use vga::Writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        colp: 0,
        rowp: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
        color_code: 0xf,
    });
}

impl Write for vga::Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
  ($($text:expr),*) => {
    WRITER.lock().write_fmt(format_args!($($text),*));
  }
}
