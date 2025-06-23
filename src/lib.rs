#![cfg_attr(not(test), no_std)] // Use no_std except for tests

#[cfg(test)]
extern crate std;

pub mod font8x8_basic;
pub mod frame_buffer;
pub mod mailbox;
pub mod text_buffer;
pub mod timer;
