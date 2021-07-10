#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

use core::panic::PanicInfo;

use rkos::{
    memory::{self},
    vga::{Charecter, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER},
};

use bootloader::{entry_point, BootInfo};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

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

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));

    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
