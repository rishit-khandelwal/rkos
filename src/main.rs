#![no_std]
#![no_main]

mod mods;

use core::fmt::Write;
use core::panic::PanicInfo;

use mods::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
