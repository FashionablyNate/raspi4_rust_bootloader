#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

use raspi4_rust_bootloader::{
    frame_buffer::FrameBuffer, mailbox::Mailbox, text_buffer::TextBuffer, timer::Timer,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;

global_asm!(
    include_str!("boot.s"),
    CONST_CORE_ID_MASK = const 0b11
);

const MAILBOX_BASE: usize = 0xFE00B880;

#[unsafe(no_mangle)]
pub extern "C" fn _start_rust() -> ! {
    let mut mailbox = Mailbox::new(MAILBOX_BASE);
    let mut fb = FrameBuffer::new(&mut mailbox).expect("Failed to create frame buffer");
    let mut tb = TextBuffer::<14, 26, Mailbox>::new(&mut fb, 100, 100, 8, 0x282828);
    let mut timer = Timer::new(1000);

    let mut counter = 0;
    loop {
        if timer.elapsed() {
            let _ = writeln!(tb, "{} seconds", counter);
            counter += 1;
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut mailbox = Mailbox::new(MAILBOX_BASE);
    let mut fb = FrameBuffer::new(&mut mailbox).expect("Failed to create frame buffer");
    let mut tb = TextBuffer::<14, 26, Mailbox>::new(&mut fb, 100, 100, 8, 0xFF0000);
    let _ = write!(tb, "PANIC:");
    if let Some(loc) = info.location() {
        let _ = write!(tb, "{}:{}: ", loc.file(), loc.line());
    }
    let _ = write!(tb, "{}\n", info.message());

    loop {}
}
