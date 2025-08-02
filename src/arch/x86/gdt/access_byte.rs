// access byte
// 7 6 5  4 3  2   1   0
// P DPL  S E  DC  RW  A
// P present bit (1) for any valid segment
// DPL descriptor privilege level 0 = highest 3 = lowest privilege(user app)
// S descriptor byte 0 system segment 1 code or data segment
// E executable bit 0 data segment 1 code segment
// DC direction/conforming bit
// RW
// A

#[derive(Clone, Copy)]
pub struct AccessByte(pub u8);

// privilege level
#[derive(Debug)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SegmentType {
    SystemSegment = 0,
    CodeOrData = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SegmentDirection {
    Normal = 0,
    ConformingOrExpandDown = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Executable {
    Data = 0,
    Code = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum ReadableWriteable {
    No = 0,
    Yes = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Accessed {
    No = 0,
    Yes = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Present {
    No = 0,
    Yes = 1,
}

pub struct AccessByteBuilder {
    pub value: u8,
}

impl AccessByteBuilder {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn present(mut self, p: Present) -> Self {
        self.value |= (p as u8) << 7;
        self
    }

    pub fn dpl(mut self, level: PrivilegeLevel) -> Self {
        self.value |= (level as u8) << 5;
        self
    }

    pub fn system(mut self, s: SegmentType) -> Self {
        self.value |= (s as u8) << 4;
        self
    }

    pub fn executable(mut self, ex: Executable) -> Self {
        self.value |= (ex as u8) << 3;
        self
    }

    pub fn direction(mut self, d: SegmentDirection) -> Self {
        self.value |= (d as u8) << 2;
        self
    }

    pub fn readable_writeable(mut self, rw: ReadableWriteable) -> Self {
        self.value |= (rw as u8) << 1;
        self
    }

    pub fn accessed(mut self, a: Accessed) -> Self {
        self.value |= (a as u8) << 0;
        self
    }

    pub fn build(self) -> AccessByte {
        AccessByte(self.value)
    }
}


use core::fmt;

impl fmt::Debug for AccessByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        let present = if (val & (1 << 7)) != 0 { Present::Yes } else { Present::No };
        let dpl = match (val >> 5) & 0b11 {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => unreachable!(),
        };
        let system = if (val & (1 << 4)) != 0 { SegmentType::CodeOrData } else { SegmentType::SystemSegment };
        let executable = if (val & (1 << 3)) != 0 { Executable::Code } else { Executable::Data };
        let direction = if (val & (1 << 2)) != 0 { SegmentDirection::ConformingOrExpandDown } else { SegmentDirection::Normal };
        let rw = if (val & (1 << 1)) != 0 { ReadableWriteable::Yes } else { ReadableWriteable::No };
        let accessed = if (val & (1 << 0)) != 0 { Accessed::Yes } else { Accessed::No };

        f.debug_struct("AccessByte")
            .field("present", &present)
            .field("dpl", &dpl)
            .field("system", &system)
            .field("executable", &executable)
            .field("direction", &direction)
            .field("readable_writeable", &rw)
            .field("accessed", &accessed)
            .finish()
    }
}
