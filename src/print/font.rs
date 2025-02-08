use pamos_macros::load_psf;

pub const PSF: PsfFont = load_psf!("src/print/font.psf");

pub struct PsfFont {
    height: u8,
    data: [&'static [u8]; 256],
}

impl PsfFont {
    pub fn glyph(&self, c: u8) -> &[u8] {
        self.data[c as usize]
    }

    pub fn height(&self) -> u8 {
        self.height
    }
}
