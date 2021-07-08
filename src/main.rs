#![no_std]
#![no_main]

mod vga;

use core::{
    fmt::{self, write},
    panic::PanicInfo,
};

use vga::{Buffer, Color, VGAWriter};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = VGAWriter {
        cpos: 0,
        rpos: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    print!(writer, "{}", 1.0 / 3.0);

    loop {}
}
