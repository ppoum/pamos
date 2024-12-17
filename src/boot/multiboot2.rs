//! Custom implementation of a semi-MB2 information sharing protocol
//! Just like MB2, our protocol contains a header and a modular series of tags
//! containing information regarding the machine. When entering the kernel, `rdi`
//! should contain the MB2 magic (0x36D76289) to indicate that `rsi` is pointing
//! to a list of MB2 tags.
//!
//! ## Header
//!     +-------------------+
//! u32 | magic (0x36D76289)|
//!     +-------------------+
//!
//! * `magic`: The start of the MB2 list should start with the MB2 magic.
//!
//! ## Common tag fields
//!     +-------------------+
//! u32 | type              |
//! u32 | size              |
//!     | ...               |
//!     +-------------------+
//!
//! * `type`: A unique ID representing the type of the tag.
//! * `size`: Size of the tag. Each tag should be padded to be aligned to 8 bytes.
//!
//! ## End
//!     +-------------------+
//! u32 | type = 0x0        |
//! u32 | size = 8          |
//!     +-------------------+
//!
//! This tag has no data and is used to indicate the end of the list.
//!
//! ## Framebuffer
//!     +-------------------+
//! u32 | type = 0x1        |
//! u32 | size = 32         |
//! u64 | address           |
//! u32 | horizontal_res    |
//! u32 | vertical_res      |
//! u32 | scanline_size     |
//! u32 | pixel_format      |
//!     +-------------------+
//!
//! * `address`: Base address of the frame buffer
//! * `horizontal_res`: The # of *visual* pixels in a row
//! * `vertical_res`: The # of pixels in a column
//! * `scanline_size`: The # of pixels in a row. Scanline rows may be larger than the screen's
//!   horizontal resolution for alignment or other reasons.
//! * `pixel_format`: Enum
//!     * 0 - RGB: Memory layout is 8 bits for R, 8 for G, 8 for B and 8 that are reserved.
//!     * 1 - BGR: Same as above, but the order is BGR

pub const EAX_MB2_MAGIC: u32 = 0x36d76289;

pub struct BootInfo {
    tag_addr: usize,
}

impl BootInfo {
    /// # SAFETY
    /// `addr` must point to a MB2 data struct.
    pub unsafe fn try_from_addr(addr: usize) -> Result<Self, &'static str> {
        let magic = *(addr as *const u32);
        if magic != EAX_MB2_MAGIC {
            return Err("Invalid MB2 magic");
        }
        Ok(Self { tag_addr: addr + 4 })
    }
}

#[repr(C)]
struct GenericTag {
    tag_type: u32,
    size: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FramebufferTag {
    pub tag_type: u32,
    pub size: u32,
    pub address: u64,
    pub horizontal_res: u32,
    pub vertical_res: u32,
    pub scanline_size: u32,
    pub pixel_format: u32,
}

#[derive(Debug, Default)]
pub struct TagCollector {
    pub framebuffer: Option<&'static FramebufferTag>,
}

impl TagCollector {
    pub fn try_new(boot_info: &BootInfo) -> Result<Self, &'static str> {
        let mut collector = Self::default();
        let mut index = 0;
        loop {
            // SAFETY: Each tag should be valid until we see the end tag
            let tag = unsafe {
                let base = boot_info.tag_addr + index;
                &*(base as *const GenericTag)
            };

            match (tag.tag_type, tag.size) {
                (0, 8) => break, // End tag
                (1, 32) => {
                    // Framebuffer
                    debug_assert!(collector.framebuffer.is_none());
                    // SAFETY: type and size match, should be a valid framebuffer tag
                    let cast = unsafe { &*(tag as *const _ as *const FramebufferTag) };
                    collector.framebuffer = Some(cast);
                }
                _ => return Err("Unknown MB2 tag"),
            }

            index += tag.size as usize;
        }
        Ok(collector)
    }
}
