#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod mailbox;
mod frame_buffer;

use crate::{mailbox::Mailbox, frame_buffer::FrameBuffer};

// MMIO addresses
const MAILBOX_BASE: usize = 0xFE00B880;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut mailbox = Mailbox::new(MAILBOX_BASE);

    if let Some(fb) = FrameBuffer::new(&mut mailbox) {
        fb.clear(0x0000FFFF);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

