//! NAS Keyset Encoding.

struct NasKeySetId {
    sec_context: u8,
    identifier: KeysetId,
}

#[repr(u8)]
pub enum SecurityContextType {
    Native = 0x00,
    Mapped = 0x01,
}
