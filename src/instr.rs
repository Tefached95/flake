pub struct Instruction {
    pub msb: u8,
    pub x: u8,
    pub y: u8,
    pub nibble: u8,
    pub kk_byte: u8,
    pub address: u16,
}

impl Instruction {
    pub fn from_u16(cmd: u16) -> Self {
        Instruction {
            msb: (cmd >> 12) as u8,
            x: ((cmd >> 8) & 0xF) as u8,
            y: ((cmd >> 4) & 0xF) as u8,
            kk_byte: (cmd & 0xFF) as u8,
            nibble: (cmd & 0xF) as u8,
            address: cmd & 0xFFF,
        }
    }
}

impl core::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("msb", &format_args!("{:#01X?}", self.msb))
            .field("x", &format_args!("{:#01X?}", self.x))
            .field("y", &format_args!("{:#01X?}", self.y))
            .field("nibble", &format_args!("{:#01X?}", self.nibble))
            .field("kk_byte", &format_args!("{:#02X?}", self.kk_byte))
            .field("address", &format_args!("{:#03X?}", self.address))
            .finish()
    }
}
