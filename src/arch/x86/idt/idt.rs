use core::option;

use crate::arch::x86::idt::type_attributes::{GateType, Present, PrivilegeLevel, TypeAttributes, TypeAttributesBuilder};

const MAX_X86_IDT_ENTRIES: usize = 256;
const KERNEL_CODE_CS: u16 = 0x08;

// #[unsafe(naked)]
extern "C" fn stub_interrupt_handler() {
    unsafe {
        core::arch::asm!(
            "mov al, 0x20",       // EOI command
            "out 0x20, al",       // send to master PIC
            "iret",
            options(noreturn)
        );
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    offset_1: u16,
    selector: u16,
    zero: u8,
    type_attributes: TypeAttributes,
    offset_2: u16,
}

impl IDTEntry {
    pub fn new(handler: extern "C" fn(), selector: u16, type_attributes: TypeAttributes) -> Self {
        let addr = handler as usize;
        Self{
            offset_1: (addr & 0xFFFF) as u16,
            selector,
            zero: 0,
            type_attributes,
            offset_2: ((addr >> 16) & 0xFFFF) as u16,
        }
    }
}

#[repr(C, packed)]
pub struct IDTR {
    pub limit: u16,
    pub base: u32,
}

#[unsafe(no_mangle)]
pub static mut IDT: [IDTEntry; MAX_X86_IDT_ENTRIES] = [IDTEntry {
    offset_1: 0,
    selector: 0,
    zero: 0,
    type_attributes: TypeAttributes(0),
    offset_2: 0,
}; MAX_X86_IDT_ENTRIES];


fn load_idt(idtr: &IDTR){
    unsafe{
        core::arch::asm!(
            "lidt [{0}]",
            in (reg) idtr,
            options(nostack, preserves_flags)
        );
    }
}

pub fn idt_init() {
    unsafe {
        for i in 0..MAX_X86_IDT_ENTRIES {
            let attr = TypeAttributesBuilder::new()
            .present(Present::Yes)
            .dpl(PrivilegeLevel::Ring0)
            .gate_type(GateType::InterruptGate32)
            .build();

            IDT[i] = IDTEntry::new(stub_interrupt_handler, KERNEL_CODE_CS, attr);
        }

        let idtr = IDTR{
            limit: (core::mem::size_of::<[IDTEntry; MAX_X86_IDT_ENTRIES]>() -1) as u16,
            base: (&raw mut IDT as *mut _ as u32),
        };
        
        load_idt(&idtr);
    }
}
