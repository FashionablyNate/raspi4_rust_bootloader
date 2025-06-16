
use core::ptr::{read_volatile, write_volatile};

const MAILBOX_READ_OFFSET: usize = 0x00;
const MAILBOX_STATUS_OFFSET: usize = 0x18;
const MAILBOX_WRITE_OFFSET: usize = 0x20;
const MAILBOX_FULL: u32 = 1 << 31;
const MAILBOX_EMPTY: u32 = 1 << 30;

pub struct Mailbox {
    base_addr: usize,
}

impl Mailbox {
    pub const fn new(mailbox_address: usize) -> Self {
        Mailbox {
            base_addr: mailbox_address,
        }
    }

    pub fn call(&self, channel: u8, buffer: *mut u32) -> bool {
        let msg = (buffer as usize & !0xF) | (channel as usize & 0xF);

        unsafe {
            while read_volatile((self.base_addr + MAILBOX_STATUS_OFFSET) as *const u32) & MAILBOX_FULL != 0 {}
            write_volatile((self.base_addr + MAILBOX_WRITE_OFFSET) as *mut u32, msg as u32);

            loop {
                while read_volatile((self.base_addr + MAILBOX_STATUS_OFFSET) as *const u32) & MAILBOX_EMPTY != 0 {}
                let resp = read_volatile((self.base_addr + MAILBOX_READ_OFFSET) as *const u32);
                if resp as usize == msg {
                    return true;
                }
            }
        }
    }
}
