use crate::{font8x8_basic::FONT8X8_BASIC, frame_buffer::FrameBuffer};

pub struct TextBuffer<'a, const ROWS: usize, const COLS: usize> {
    fb: &'a mut FrameBuffer,
    cursor_x: usize,
    cursor_y: usize,
    offset_x: usize,
    offset_y: usize,
    font_size: usize,
    glyph_size: usize,
    buffer: [[char; COLS]; ROWS],
    dirty_line: bool,
    font_color: u32,
    background_color: u32,
}

impl<'a, const ROWS: usize, const COLS: usize> TextBuffer<'a, ROWS, COLS> {
    pub fn new(
        fb: &'a mut FrameBuffer,
        offset_x: usize,
        offset_y: usize,
        font_size: usize,
        background_color: u32,
    ) -> Self {
        let glyph_size = 8 * font_size;
        Self {
            fb,
            cursor_x: 0,
            cursor_y: 0,
            offset_x,
            offset_y,
            font_size,
            glyph_size,
            buffer: [[' '; COLS]; ROWS],
            dirty_line: false,
            font_color: (background_color ^ 0xFFFFFF) & 0xFFFFFF,
            background_color,
        }
    }

    fn draw_char_at(&mut self, row: usize, col: usize, ch: char) {
        let x = col * self.glyph_size + self.offset_x;
        let y = row * self.glyph_size + self.offset_y;

        self.fb.clear_area(
            (x, x + self.glyph_size),
            (y, y + self.glyph_size),
            self.background_color,
        );

        if (' '..='~').contains(&ch) {
            self.fb.draw_glyph(
                x,
                y,
                ch as u8,
                self.font_color,
                self.font_size,
                &FONT8X8_BASIC,
            );
        }
    }

    pub fn redraw(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                self.draw_char_at(row, col, self.buffer[row][col]);
            }
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..ROWS {
            self.buffer[row - 1] = self.buffer[row];
        }
        self.buffer[ROWS - 1] = [' '; COLS];
    }
}

impl<'a, const ROWS: usize, const COLS: usize> core::fmt::Write for TextBuffer<'a, ROWS, COLS> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            if self.dirty_line {
                self.scroll_up();
                self.dirty_line = false;
            }
            match ch {
                '\n' => {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                    if self.cursor_y >= ROWS {
                        self.dirty_line = true;
                        self.cursor_y = ROWS - 1;
                    }
                }
                '\r' => {
                    self.cursor_x = 0;
                }
                ' '..='~' => {
                    self.buffer[self.cursor_y][self.cursor_x] = ch;
                    self.cursor_x += 1;
                    if self.cursor_x >= COLS {
                        self.cursor_x = 0;
                        self.cursor_y += 1;
                        if self.cursor_y >= ROWS {
                            self.dirty_line = true;
                            self.cursor_y = ROWS - 1;
                        }
                    }
                }
                _ => {}
            }
        }
        self.redraw();
        self.fb.swap_buffer();
        Ok(())
    }
}
