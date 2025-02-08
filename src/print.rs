use crate::boot::multiboot2::FramebufferTag;

mod font;

// NOTE: This module will be a weird mix of print utility and tty for a while until I decide to
// split them into separate modules.

static mut FB: Framebuffer = Framebuffer {
    addr: 0 as *mut _,
    scanline_size: 0,
    row_size: 0,
    col_size: 0,
    max_row: 0,
    max_col: 0,
    row_idx: 0,
    col_idx: 0,
};

struct Framebuffer {
    addr: *mut u32,
    scanline_size: u32,
    row_size: u32,
    col_size: u32,
    max_row: u32,
    max_col: u32,
    row_idx: u32,
    col_idx: u32,
}

impl Framebuffer {
    pub fn char_coord_to_ptr(&self, row: u32, col: u32) -> *mut u32 {
        assert!(row < self.max_row);
        assert!(col < self.max_col);
        let offset = row * self.scanline_size * self.row_size + col * self.col_size;

        // SAFETY: Offset will never be bigger than the framebuffer due to the row and col bounds.
        unsafe { self.addr.add(offset as usize) }
    }

    pub fn current_char_ptr(&self) -> *mut u32 {
        self.char_coord_to_ptr(self.row_idx, self.col_idx)
    }

    pub fn increment_cursor(&mut self) {
        if self.col_idx == self.max_col - 1 {
            self.col_idx = 0;
            self.row_idx += 1;
            // FIXME: Implement "scrolling" later
            assert!(self.row_idx < self.max_row)
        } else {
            self.col_idx += 1;
        }
    }

    pub fn set_cursor(&mut self, row: u32, col: u32) {
        debug_assert!(row < self.max_row);
        debug_assert!(col < self.max_col);
        self.row_idx = row;
        self.col_idx = col;
    }

    /// Resets the column without changing the row. Used for carriage return (`\r`)
    pub fn carriage_return(&mut self) {
        self.col_idx = 0;
    }

    /// Increments the row and set the column to 0. Used for newlines (`\n`)
    pub fn newline(&mut self) {
        // FIXME: Scrolling
        self.row_idx += 1;
        self.carriage_return();
    }
}

/// # SAFETY
/// This function should be called *once* before all other functions in this module.
pub unsafe fn initialize_fb_from_mb2_tag(tag: &FramebufferTag) {
    let row_size = font::PSF.height() as u32;
    let col_size = 8; // PSF1 always has a width of 8 pixels
    FB = Framebuffer {
        addr: tag.address as *mut _,
        scanline_size: tag.scanline_size,
        row_size,
        col_size,
        max_row: tag.vertical_res / row_size,
        max_col: tag.horizontal_res / col_size,
        row_idx: 0,
        col_idx: 0,
    }
}

/// Wrapper to extract the Framebuffer struct out of the unsafe mutable static variable.
fn fb_as_ref() -> &'static mut Framebuffer {
    #[allow(static_mut_refs)]
    unsafe {
        &mut FB
    }
}

pub fn print_ascii_char(char: u8) {
    let glyph = font::PSF.glyph(char);
    let fb = fb_as_ref();

    if char == b'\r' {
        fb.carriage_return();
        return;
    } else if char == b'\n' {
        fb.newline();
        return;
    }

    let base = fb.current_char_ptr();
    for (i, row) in glyph.iter().enumerate() {
        for j in 0..8 {
            let idx = i * fb.scanline_size as usize + (8 - j);
            let bit_set = (row & (0x1 << j)) != 0;
            if bit_set {
                unsafe { *(base.add(idx)) = 0xFFFFFFFF };
            } else {
                unsafe { *(base.add(idx)) = 0 };
            }
        }
    }
    fb.increment_cursor();
}

pub fn print_str(s: &str) {
    let mut buf = [0];
    for c in s.chars() {
        debug_assert!(c.is_ascii());
        c.encode_utf8(&mut buf);
        print_ascii_char(buf[0]);
    }
}
