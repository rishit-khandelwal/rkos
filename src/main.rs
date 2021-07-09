#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

use core::panic::PanicInfo;

use rkos::vga::{Charecter, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rkos::init();

    let mut writer = WRITER.lock();

    let s = "RK-OS";

    for l in 0..BUFFER_WIDTH {
        writer.buffer.chars[BUFFER_HEIGHT - 1][l] = Charecter {
            ascii_char: b'\0',
            color_code: ColorCode::new(Color::White, Color::Blue),
        };
    }

    let mut p = 0;
    for l in ((BUFFER_WIDTH - s.len()) / 2)..(BUFFER_WIDTH + s.len()) / 2 {
        writer.buffer.chars[BUFFER_HEIGHT - 1][l] = Charecter {
            ascii_char: s.bytes().nth(p).unwrap(),
            color_code: ColorCode::new(Color::White, Color::Blue),
        };
        p += 1;
    }

    unsafe { WRITER.force_unlock() };

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
