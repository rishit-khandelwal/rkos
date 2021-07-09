#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

use core::panic::PanicInfo;

use rkos::{
    memory::active_level_4_table,
    println,
    vga::{Charecter, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER},
};

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(bootinfo: &'static BootInfo) -> ! {
    rkos::init();

    let mut writer = WRITER.lock();

    let s = "RK-OS made by Rishit Khandelwal (github.com/rishit-khandelwal)";

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

    let phys_mem_offset = VirtAddr::new(bootinfo.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 entry {}: {:?}", i, entry);
        }
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
