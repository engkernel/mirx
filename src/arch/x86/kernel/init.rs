use crate::arch::x86::{gdt::gdt::gdt_init, idt::idt};

pub fn x86_init() {
    idt::disable_interrupts();
    gdt_init();
}
