#[derive(Debug)]
#[repr(u8)]
pub enum Present {
    No = 0,
    Yes = 1,
}

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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GateType {
    TaskGate = 0b0101,
    InterruptGate16 = 0b0110,
    TrapGate16 = 0b0111,
    InterruptGate32 = 0b1110,
    TrapGate32 = 0b1111,
}

#[derive(Clone, Copy)]
pub struct TypeAttributes(pub u8);

pub struct TypeAttributesBuilder{
    value: u8,
}

impl TypeAttributesBuilder {
    pub fn new() -> Self {
        Self{ value: 0}
    }

    pub fn present(mut self, p: Present) -> Self {
        self.value |= (p as u8) << 7;
        self
    }

    pub fn dpl(mut self, level: PrivilegeLevel) -> Self {
        self.value |= (level as u8 & 0b11) << 5;
        self
    }

    pub fn gate_type(mut self, gate: GateType) -> Self {
        self.value |= gate as u8;
        self
    }

    pub fn build(self) -> TypeAttributes {
        TypeAttributes(self.value)
    }
}


