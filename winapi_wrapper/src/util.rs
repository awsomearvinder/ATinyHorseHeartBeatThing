pub struct RGBA(pub u8,pub u8,pub u8,pub u8);
impl From<RGBA> for (u32, u8) {
    fn from(r: RGBA) -> Self {
        let mut o: u32 = 0;
        o |= r.0 as u32;
        o <<= 8;
        o |= r.1 as u32;
        o <<= 8;
        o |= r.2 as u32;
        (o, r.3)
    }
}