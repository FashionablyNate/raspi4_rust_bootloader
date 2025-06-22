#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

mod font8x8_basic;
mod frame_buffer;
mod mailbox;
mod text_buffer;

use crate::{frame_buffer::FrameBuffer, mailbox::Mailbox, text_buffer::TextBuffer};

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

    if let Some(mut fb) = FrameBuffer::new(&mut mailbox) {
        fb.clear(0x0000FFFF);
        if let Ok(mut tb) = TextBuffer::<13, 26>::new(&mut fb, (100, 1820), (100, 980), 8) {
            let _ = writeln!(tb, "Hello world!A");
            let _ = writeln!(tb, "Hello world!B");
            let _ = writeln!(tb, "Hello world!C");
            let _ = writeln!(tb, "Hello world!D");
            let _ = writeln!(tb, "Hello world!E");
            let _ = writeln!(tb, "Hello world!F");
            let _ = writeln!(tb, "Hello world!G");
            let _ = writeln!(tb, "Hello world!H");
            let _ = writeln!(tb, "Hello world!I");
            let _ = writeln!(tb, "Hello world!J");
            let _ = writeln!(tb, "Hello world!K");
            let _ = writeln!(tb, "Hello world!L");
            let _ = writeln!(tb, "Hello world!M");
            let _ = writeln!(tb, "Hello world!N");
            let _ = writeln!(tb, "Hello world!O");
            let _ = writeln!(tb, "Hello world!P");
            let _ = writeln!(tb, "Hello world!Q");
        } else {
            panic!("Failed to initialize text buffer!");
        }
    }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut mailbox = Mailbox::new(MAILBOX_BASE);

    if let Some(mut fb) = FrameBuffer::new(&mut mailbox) {
        fb.clear(0x00FF0000);
        if let Ok(mut tb) = TextBuffer::<13, 26>::new(&mut fb, (100, 1820), (100, 980), 8) {
            let _ = write!(tb, "PANIC:");
            if let Some(loc) = info.location() {
                let _ = write!(tb, "{}:{}: ", loc.file(), loc.line());
            }
            let _ = write!(tb, "{}\n", info.message());
        }
    }

    loop {}
}
