#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
mod vga;
use vga::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // print!("Hello!");

    let mut w = vga::Writer {
        colp: 0,
        rowp: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color_code: 0xf,
    };

    write!(w, "{}", 1.0 / 3.0).unwrap();

    loop {}
}
