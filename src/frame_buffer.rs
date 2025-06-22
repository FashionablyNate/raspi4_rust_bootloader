use core::ptr::write_volatile;

use crate::mailbox::Mailbox;

const CHANNEL_FRAMEBUFFER: u8 = 8;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const MAILBOX_BASE: usize = 0xFE00B880;

#[repr(C, align(16))]
struct FrameBufferInitMailbox {
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

    // Set virtual offset
    tag_set_virtual_offset: u32,
    tag_virtual_offset_bufsize: u32,
    tag_virtual_offset_len: u32,
    offset_x: u32,
    offset_y: u32,

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

static mut FB_MAILBOX: FrameBufferInitMailbox = FrameBufferInitMailbox {
    // Size of the whole message in bytes
    size: size_of::<FrameBufferInitMailbox>() as u32,
    code: 0x00000000,

    // Set physical size
    tag_set_physical_size: 0x00048003,
    tag_physical_size_bufsize: 8,
    tag_physical_size_len: 8,
    physical_width: WIDTH,
    physical_height: HEIGHT,

    // Set virtual size
    tag_set_virtual_size: 0x00048004,
    tag_virtual_size_bufsize: 8,
    tag_virtual_size_len: 8,
    virtual_width: WIDTH,
    virtual_height: HEIGHT * 2,

    // Set virtual offset
    tag_set_virtual_offset: 0x00048009,
    tag_virtual_offset_bufsize: 8,
    tag_virtual_offset_len: 8,
    offset_x: 0,
    offset_y: 0, // Will change dynamically

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
    end_tag: 0,
};

#[repr(C, align(16))]
struct FrameBufferSetOffsetMailbox {
    size: u32,
    code: u32,
    tag_set_virtual_offset: u32,
    tag_bufsize: u32,
    tag_len: u32,
    offset_x: u32,
    offset_y: u32,
    end_tag: u32,
}

pub struct FrameBuffer {
    pub ptr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    mailbox: Mailbox,
    current_offset: u32,
}

impl FrameBuffer {
    pub fn new() -> Option<Self> {
        let mailbox = Mailbox::new(MAILBOX_BASE);
        if mailbox.call(
            CHANNEL_FRAMEBUFFER,
            &raw mut FB_MAILBOX as *mut _ as *mut u32,
        ) {
            let (ptr, pitch, width, height) = unsafe {
                let ptr = (FB_MAILBOX.fb_ptr & 0x3FFFFFFF) as *mut u32;
                let pitch = FB_MAILBOX.pitch as usize;
                let width = FB_MAILBOX.virtual_width as usize;
                let height = FB_MAILBOX.virtual_height as usize;
                (ptr, pitch, width, height)
            };
            let fb = FrameBuffer {
                ptr,
                width,
                height,
                pitch,
                mailbox,
                current_offset: 0,
            };

            fb.clear(0x282828);

            return Some(fb);
        }

        None
    }

    pub fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }

    pub fn clear_area(&self, x_area: (usize, usize), y_area: (usize, usize), color: u32) {
        for y in y_area.0..y_area.1 {
            for x in x_area.0..x_area.1 {
                self.draw_pixel(x, y, color);
            }
        }
    }

    pub fn draw_glyph<const GLYPH_HEIGHT: usize>(
        &self,
        x: usize,
        y: usize,
        ch: u8,
        color: u32,
        scale: usize,
        font: &[[u8; GLYPH_HEIGHT]; 128],
    ) {
        if ch as usize >= font.len() {
            return;
        }

        let glyph = &font[ch as usize];

        for (row_idx, row_bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if (row_bits >> col) & 1 != 0 {
                    let base_x = x + col * scale;
                    let base_y = y + row_idx * scale;

                    for dy in 0..scale {
                        for dx in 0..scale {
                            self.draw_pixel(base_x + dx, base_y + dy, color);
                        }
                    }
                }
            }
        }
    }

    pub fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let adjusted_y = y.saturating_add(self.current_offset as usize);
            let offset = adjusted_y
                .checked_mul(self.pitch / 4)
                .and_then(|row| row.checked_add(x));
            if let Some(offset) = offset {
                unsafe {
                    write_volatile(self.ptr.add(offset), color);
                }
            }
        }
    }

    pub fn swap_buffer(&mut self) {
        self.set_virtual_offset(self.current_offset);
        self.current_offset = if self.current_offset == 0 { HEIGHT } else { 0 };
    }

    fn set_virtual_offset(&self, y_offset: u32) {
        let mut mb = FrameBufferSetOffsetMailbox {
            size: core::mem::size_of::<FrameBufferSetOffsetMailbox>() as u32,
            code: 0,
            tag_set_virtual_offset: 0x00048009,
            tag_bufsize: 8,
            tag_len: 8,
            offset_x: 0,
            offset_y: y_offset,
            end_tag: 0,
        };

        self.mailbox
            .call(CHANNEL_FRAMEBUFFER, &mut mb as *mut _ as *mut u32);
    }
}
