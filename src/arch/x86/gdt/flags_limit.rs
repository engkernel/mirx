// flags and limit
// limit 4 bits 0xffff
// flags 3 2  1  0
//       G DB L  Reserved
// G granularity flag, 0 byte granularity, 1 4KiB granularity(page granularity)
// DB 0 16-bit protected mode segment, 1 32-bit protected mode segment
// L 1 if 64-bit code segment, if set db should be clear
// Reserved is zero
#[derive(Clone, Copy)]
pub struct FlagsLimitByte(pub u8);

#[derive(Debug)]
#[repr(u8)]
pub enum Granularity {
    Byte = 0,
    Page4KiB = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum OperandSize {
    Bit16 = 0,
    Bit32 = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum LongMode {
    Disabled = 0,
    Enabled = 1,
}

pub struct FlagsLimitBuilder {
    pub flags: u8,
    pub limit: u8,
}

impl FlagsLimitBuilder {
    pub fn new() -> Self {
        Self {
            flags: 0,
            limit: 0xF,
        }
    }

    pub fn granularity(mut self, g: Granularity) -> Self {
        self.flags |= (g as u8) << 7;
        self
    }

    pub fn db(mut self, db: OperandSize) -> Self {
        self.flags |= (db as u8) << 6;
        self
    }

    pub fn long_mode(mut self, m: LongMode) -> Self {
        match m {
            LongMode::Enabled => {
                self.flags |= 1 << 5;
                self.flags &= !(1 << 6);
            }
            LongMode::Disabled => {
                self.flags &= !(1 << 5);
            }
        }
        self
    }

    pub fn limit(mut self, val: u8) -> Self {
        self.limit = val & 0x0F;
        self
    }

    pub fn build(self) -> FlagsLimitByte {
        FlagsLimitByte((self.flags & 0xF0) | (self.limit & 0x0F))
    }
}


use core::fmt;

impl fmt::Debug for FlagsLimitByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gran = if (self.0 & 0x80) != 0 {
            Granularity::Page4KiB
        } else {
            Granularity::Byte
        };
        let db = if (self.0 & 0x40) != 0 {
            OperandSize::Bit32
        } else {
            OperandSize::Bit16
        };
        let long_mode = if (self.0 & 0x20) != 0 {
            LongMode::Enabled
        } else {
            LongMode::Disabled
        };
        let limit = self.0 & 0x0F;

        f.debug_struct("FlagsLimitByte")
            .field("granularity", &gran)
            .field("db", &db)
            .field("long_mode", &long_mode)
            .field("limit", &format_args!("{:#X}", limit))
            .finish()
    }
}
