use std::{fs::File, io::Read, path::Path};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn load_psf(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let str = input_str.value();
    let psf_path = Path::new(&str);

    match load_psf_inner(psf_path) {
        Ok(stream) => stream,
        Err(msg) => syn::Error::new(input_str.span(), msg)
            .to_compile_error()
            .into(),
    }
}

fn load_psf_inner(path: &Path) -> Result<TokenStream, String> {
    if !path.exists() {
        return Err(format!("File '{}' does not exist", path.to_string_lossy()));
    }
    let mut file = File::open(path).map_err(|e| e.to_string())?;

    let mut buf = [0; 4];
    file.read_exact(&mut buf).map_err(|e| e.to_string())?;

    if buf == [0x72, 0xb5, 0x4a, 0x86] {
        return Err(String::from(
            "psf2 file format unsupported by load_psf! macro",
        ));
    }

    if buf[..2] != [0x36, 0x04] {
        return Err(String::from("Unknown file format"));
    }

    let psf_font_mode = buf[2];
    let has_512_glyphs = (psf_font_mode & 0x01) != 0;
    let _has_unicode = (psf_font_mode & 0x06) != 0;
    // NOTE: Unicode characters are ignored for now

    let height = buf[3];

    let mut glyphs: Vec<Vec<u8>> = vec![];
    let max_idx = if has_512_glyphs { 512 } else { 256 };
    for _ in 0..max_idx {
        let mut v = vec![];
        for _ in 0..height {
            let mut row_buf = [0; 1];
            file.read_exact(&mut row_buf).map_err(|e| e.to_string())?;
            v.push(row_buf[0]);
        }
        glyphs.push(v);
    }

    // Convert Vec<Vec<_>> to token stream
    let glyphs_tokens = glyphs.iter().map(|inner| quote!(&[#(#inner),*]));
    let glyphs_tokens = quote!([#(#glyphs_tokens),*]);

    Ok(quote!(crate::print::font::PsfFont {
        height: #height,
        data: #glyphs_tokens
    })
    .into())
}
