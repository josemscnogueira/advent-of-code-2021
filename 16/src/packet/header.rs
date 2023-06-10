pub struct Header {
    pub version: u8,
    pub id: u8,
}

impl Header {
    const VTABLE_VER: (usize, usize) = (0, 3);
    const VTABLE_UID: (usize, usize) = (3, 6);
    pub const VTABLE_END: usize = Self::VTABLE_UID.1;

    pub fn init(data: &[bool]) -> Self {
        Self {
            version: super::parse_bitstream(
                &data[Self::VTABLE_VER.0..Self::VTABLE_VER.1],
            ),
            id: super::parse_bitstream(
                &data[Self::VTABLE_UID.0..Self::VTABLE_UID.1],
            ),
        }
    }
}
