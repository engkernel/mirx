use crate::arch::x86::idt::type_attributes::{TypeAttributes, TypeAttributesBuilder};

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    offset_1: u16,
    selector: u16,
    zero: u8,
    type_attributes: TypeAttributes,
    offset_2: u16,
}

#[repr(C, packed)]
pub struct IDTR{
    pub limit: u16,
    pub base: u32,
}

pub static mut IDT: [IDTEntry; 256];

pub fn idt_init() {

}