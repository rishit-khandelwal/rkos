#![no_std]
#![no_main]

mod mods;

use core::fmt::Write;
use core::panic::PanicInfo;

use mods::*;

use mods::mcrs::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    WRITER.lock().color_code = 0xc;
    print!("{}", _info);
    WRITER.lock().color_code = 0xf;

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("hi!");

    loop {}
}
