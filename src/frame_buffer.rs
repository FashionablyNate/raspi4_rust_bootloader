
use core::ptr::write_volatile;

use crate::mailbox::Mailbox;

const CHANNEL_FRAMEBUFFER: u8 = 8;

#[repr(C, align(16))]
struct FrameBufferMailbox {
    size: u32,
    code: u32,

    // Set physical size
    tag_set_physical_size: u32,
    tag_physical_size_bufsize: u32,
    tag_physical_size_len: u32,
    physical_width: u32,
    physical_height: u32,

    // Set virtual size
    tag_set_virtual_size: u32,
    tag_virtual_size_bufsize: u32,
    tag_virtual_size_len: u32,
    virtual_width: u32,
    virtual_height: u32,

    // Set depth
    tag_set_depth: u32,
    tag_depth_bufsize: u32,
    tag_depth_len: u32,
    depth: u32,

    // Set pixel order (optional, but good for clarity)
    tag_set_pixel_order: u32,
    tag_pixel_order_bufsize: u32,
    tag_pixel_order_len: u32,
    pixel_order: u32,

    // Allocate framebuffer
    tag_allocate_buffer: u32,
    tag_allocate_bufsize: u32,
    tag_allocate_len: u32,
    fb_ptr: u32,
    fb_size: u32,

    // Get pitch
    tag_get_pitch: u32,
    tag_pitch_bufsize: u32,
    tag_pitch_len: u32,
    pitch: u32,

    // End tag
    end_tag: u32,
}

static mut FB_MAILBOX: FrameBufferMailbox = FrameBufferMailbox {
    // Size of the whole message in bytes
    size: size_of::<FrameBufferMailbox>() as u32,
    code: 0x00000000,

    // Set physical size
    tag_set_physical_size: 0x00048003,
    tag_physical_size_bufsize: 8,
    tag_physical_size_len: 8,
    physical_width: 1920,
    physical_height: 1080,

    // Set virtual size
    tag_set_virtual_size: 0x00048004,
    tag_virtual_size_bufsize: 8,
    tag_virtual_size_len: 8,
    virtual_width: 1920,
    virtual_height: 1080,

    // Set depth
    tag_set_depth: 0x00048005,
    tag_depth_bufsize: 4,
    tag_depth_len: 4,
    depth: 32,

    // Set pixel order (RGB)
    tag_set_pixel_order: 0x00048006,
    tag_pixel_order_bufsize: 4,
    tag_pixel_order_len: 4,
    pixel_order: 0,

    // Allocate buffer
    tag_allocate_buffer: 0x00040001,
    tag_allocate_bufsize: 8,
    tag_allocate_len: 4,
    fb_ptr: 0,
    fb_size: 0,

    // Get pitch
    tag_get_pitch: 0x00040008,
    tag_pitch_bufsize: 4,
    tag_pitch_len: 4,
    pitch: 0,

    // End tag
    end_tag: 0
};

pub struct FrameBuffer {
    pub ptr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

impl FrameBuffer {
    pub fn new(mailbox: &mut Mailbox) -> Option<Self> {
        unsafe {
            if mailbox.call(CHANNEL_FRAMEBUFFER, &raw mut FB_MAILBOX as *mut _ as *mut u32) {
                let ptr = (FB_MAILBOX.fb_ptr & 0x3FFFFFFF) as *mut u32;
                let pitch = FB_MAILBOX.pitch as usize;
                let width = FB_MAILBOX.virtual_width as usize;
                let height = FB_MAILBOX.virtual_height as usize;

                return Some(FrameBuffer { ptr, width, height, pitch })
            }
        }

        None
    }

    pub fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (y * self.pitch + x * 4) as isize;
                unsafe {
                    write_volatile(self.ptr.offset(offset / 4), color);
                }
            }
        }
    }
}
