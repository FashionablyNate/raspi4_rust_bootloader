#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod font8x8_basic;
mod frame_buffer;
mod mailbox;

use crate::{frame_buffer::FrameBuffer, mailbox::Mailbox};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;

global_asm!(
    include_str!("boot.s"),
    CONST_CORE_ID_MASK = const 0b11
);

// MMIO addresses
const MAILBOX_BASE: usize = 0xFE00B880;

#[unsafe(no_mangle)]
pub extern "C" fn _start_rust() -> ! {
    let mut mailbox = Mailbox::new(MAILBOX_BASE);

    if let Some(fb) = FrameBuffer::new(&mut mailbox) {
        fb.clear(0x0000FFFF);
        fb.draw_string(
            512,
            476,
            "I love Payton!\nShe is awesome!\nAND GREAT ~$%\n",
            0x00FF0000,
            8,
            &font8x8_basic::FONT8X8_BASIC,
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
