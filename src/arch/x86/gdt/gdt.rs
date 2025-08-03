use super::access_byte::{
    AccessByte, AccessByteBuilder, Accessed, Executable, Present, PrivilegeLevel,
    ReadableWriteable, SegmentDirection, SegmentType,
};

use super::flags_limit::{FlagsLimitBuilder, FlagsLimitByte, Granularity, LongMode, OperandSize};
use core::{arch::asm, fmt};

#[repr(C, packed)]
pub struct SegmentDescriptor {
    limit_low: u16,  // Lower 16 bits of the segment limit
    base_low: u16,   // lower 16 bits of the base
    base_middle: u8, // next 8 bits of the base
    access_byte: AccessByte,
    flags_limit: FlagsLimitByte, // flags (4 bits) and upper 4 bits of the limit
    base_high: u8,               // final 8 bits of the base
}

impl SegmentDescriptor {
    pub fn new(base: u32, limit: u32, access: AccessByte, flags: FlagsLimitByte) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access_byte: access,
            flags_limit: flags,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}


#[repr(C, packed)]
pub struct TSSDescriptor {
    limit_low: u16,  // Lower 16 bits of the segment limit
    base_low: u16,   // lower 16 bits of the base
    base_middle: u8, // next 8 bits of the base
    access_byte: AccessByte,
    flags_limit: FlagsLimitByte, // flags (4 bits) and upper 4 bits of the limit
    base_high: u8,
    base_upper: u32, // upper 32 bits of base (for 64-bit)
    reserved: u32,
}

impl TSSDescriptor {
    pub fn new(base: u64, limit: u32, access: AccessByte, flags: FlagsLimitByte) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access_byte: access,
            flags_limit: flags,
            base_high: ((base >> 24) & 0xFF) as u8,
            base_upper: ((base >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }
}

// final GDT
#[repr(C, align(8))]
pub struct GlobalDescriptorTable {
    pub null: SegmentDescriptor,
    pub kernel_code: SegmentDescriptor,
    pub kernel_data: SegmentDescriptor,
    pub user_code: SegmentDescriptor,
    pub user_data: SegmentDescriptor,
    // pub tss: TSSDescriptor,
}

#[repr(C, packed)]
pub struct GDTR {
    pub limit: u16,
    pub base: u64,
}

impl GDTR {
    pub fn new(gdt: GlobalDescriptorTable) -> Self {
        let gdtr_base = &gdt as *const _ as u64;

        let gdtr_limit = (core::mem::size_of::<GlobalDescriptorTable>()) as u16 -1;

        Self {
            limit: gdtr_limit,
            base: gdtr_base,
        }
    }
}

pub fn load_gdt(gdtr: &GDTR) {
    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) gdtr,
            options(nostack, preserves_flags),
        )
    }
}

pub fn gdt_init() {
    let null = SegmentDescriptor::new(
        0,
        0,
        AccessByteBuilder::new().build(),
        FlagsLimitBuilder::new().build(),
    );

    let kernel_code = SegmentDescriptor::new(
        0x00000000,
        0xFFFFF, // 4GB
        AccessByteBuilder::new()
            .present(Present::Yes) // P = 1
            .dpl(PrivilegeLevel::Ring0) // DPL = 0 (kernel)
            .system(SegmentType::CodeOrData) // S = 1 (code/data segment)
            .executable(Executable::Code) // E = 1 (code segment)
            .direction(SegmentDirection::Normal) // DC = 0 (conforming = 0)
            .readable_writeable(ReadableWriteable::Yes) // R = 1 should be true for code to be readable
            .accessed(Accessed::No) // A = 0
            .build(),
        FlagsLimitBuilder::new()
            .granularity(Granularity::Page4KiB)
            .db(OperandSize::Bit32)
            .long_mode(LongMode::Disabled)
            .limit(0xF)
            .build(),
    );

    let kernel_data = SegmentDescriptor::new(
        0x00000000,
        0xFFFFF, // 4GB
        AccessByteBuilder::new()
            .present(Present::Yes) // P = 1
            .dpl(PrivilegeLevel::Ring0) // DPL = 0 (kernel)
            .system(SegmentType::CodeOrData) // S = 1 (code/data segment)
            .executable(Executable::Data) // E = 1 (code segment)
            .direction(SegmentDirection::Normal) // DC = 0 (conforming = 0)
            .readable_writeable(ReadableWriteable::Yes) // R = 1 should be true for code to be readable
            .accessed(Accessed::No) // A = 0
            .build(),
        FlagsLimitBuilder::new()
            .granularity(Granularity::Page4KiB)
            .db(OperandSize::Bit32)
            .long_mode(LongMode::Disabled)
            .limit(0xF)
            .build(),
    );

    let user_code = SegmentDescriptor::new(
        0x00000000,
        0xFFFFF, // 4GB
        AccessByteBuilder::new()
            .present(Present::Yes) // P = 1
            .dpl(PrivilegeLevel::Ring3) // DPL = 3 (user)
            .system(SegmentType::CodeOrData) // S = 1 (code/data segment)
            .executable(Executable::Code) // E = 1 (code segment)
            .direction(SegmentDirection::Normal) // DC = 0 (conforming = 0)
            .readable_writeable(ReadableWriteable::Yes) // R = 1 should be true for code to be readable
            .accessed(Accessed::No) // A = 0
            .build(),
        FlagsLimitBuilder::new()
            .granularity(Granularity::Page4KiB)
            .db(OperandSize::Bit32)
            .long_mode(LongMode::Disabled)
            .limit(0xF)
            .build(),
    );

    let user_data = SegmentDescriptor::new(
        0x00000000,
        0xFFFFF, // 4GB
        AccessByteBuilder::new()
            .present(Present::Yes) // P = 1
            .dpl(PrivilegeLevel::Ring3) // DPL = 3 (user)
            .system(SegmentType::CodeOrData) // S = 1 (code/data segment)
            .executable(Executable::Data) // E = 1 (code segment)
            .direction(SegmentDirection::Normal) // DC = 0 (conforming = 0)
            .readable_writeable(ReadableWriteable::Yes) // R = 1 should be true for code to be readable
            .accessed(Accessed::No) // A = 0
            .build(),
        FlagsLimitBuilder::new()
            .granularity(Granularity::Page4KiB)
            .db(OperandSize::Bit32)
            .long_mode(LongMode::Disabled)
            .limit(0xF)
            .build(),
    );

    let tss = TSSDescriptor::new(
        0,
        0,
        AccessByteBuilder::new().build(),
        FlagsLimitBuilder::new().build(),
    );

    let gdt = GlobalDescriptorTable {
        null,
        kernel_code,
        kernel_data,
        user_code,
        user_data,
        // tss
    };

    let gdtr = GDTR::new(gdt);

    load_gdt(&gdtr);
}



impl fmt::Debug for SegmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Decode limit
        let limit = ((self.flags_limit.0 as u32 & 0x0F) << 16) | self.limit_low as u32;
        // Decode base
        let base = (self.base_high as u32) << 24
            | (self.base_middle as u32) << 16
            | (self.base_low as u32);
        // Decode access_byte bits
        let access = self.access_byte.0;
        let flags = self.flags_limit.0;

        f.debug_struct("SegmentDescriptor")
            .field("Base", &format_args!("{:#010X}", base))
            .field("Limit", &format_args!("{:#X}", limit))
            .field("AccessByte", &self.access_byte)
            .field("Flags+Limit", &self.flags_limit)
            .field("  Limit High (4 bits)", &(flags & 0x0F))
            .finish()
    }
}