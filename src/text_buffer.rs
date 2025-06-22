use crate::{font8x8_basic::FONT8X8_BASIC, frame_buffer::FrameBuffer};

pub enum TextBufferError {
    TextBufferInitFailed,
}

pub struct TextBuffer<'a, const ROWS: usize, const COLS: usize> {
    fb: &'a mut FrameBuffer,
    cursor_x: usize,
    cursor_y: usize,
    offset_x: usize,
    offset_y: usize,
    font_size: usize,
    glyph_size: usize,
    buffer: [[char; COLS]; 1000],
    dirty_line: bool,
}

impl<'a, const ROWS: usize, const COLS: usize> TextBuffer<'a, ROWS, COLS> {
    pub fn new(
        fb: &'a mut FrameBuffer,
        x_bounds: (usize, usize),
        y_bounds: (usize, usize),
        font_size: usize,
    ) -> Result<Self, TextBufferError> {
        if x_bounds.0 >= x_bounds.1 || y_bounds.0 >= y_bounds.1 {
            return Err(TextBufferError::TextBufferInitFailed);
        }
        let glyph_size = 8 * font_size;
        Ok(Self {
            fb,
            cursor_x: 0,
            cursor_y: 0,
            offset_x: x_bounds.0,
            offset_y: y_bounds.0,
            font_size,
            glyph_size,
            buffer: [[' '; COLS]; 1000],
            dirty_line: false,
        })
    }

    pub fn write_char(&mut self, ch: char) {
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
            '\r' => self.cursor_x = 0,
            ' '..'~' => {
                self.buffer[self.cursor_y][self.cursor_x] = ch;
                let x = self.cursor_x * self.glyph_size + self.offset_x;
                let y = self.cursor_y * self.glyph_size + self.offset_y;

                self.fb
                    .draw_glyph(x, y, ch as u8, 0, self.font_size, &FONT8X8_BASIC);

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

    fn redraw_line(&mut self, row: usize) {
        let y = self.offset_y + row * self.glyph_size;
        self.fb.clear_area(
            (self.offset_x, self.offset_x + COLS * self.glyph_size),
            (y, y + self.glyph_size),
            0x0000FFFF,
        );

        for (col, &ch) in self.buffer[row].iter().enumerate() {
            if ch != ' ' {
                let x = self.offset_x + col * self.glyph_size;
                self.fb
                    .draw_glyph(x, y, ch as u8, 0, self.font_size, &FONT8X8_BASIC);
            }
        }
    }

    fn scroll_up(&mut self) {
        // shift all rows up by one
        for row in 1..ROWS {
            self.buffer[row - 1] = self.buffer[row];
        }
        // clear last row
        self.buffer[ROWS - 1] = [' '; COLS];

        // redraw all rows
        for row in 0..ROWS {
            self.redraw_line(row);
        }
    }
}

impl<'a, const ROWS: usize, const COLS: usize> core::fmt::Write for TextBuffer<'a, ROWS, COLS> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_char(ch);
        }
        Ok(())
    }
}
