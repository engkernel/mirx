use crate::arch::x86::{gdt::gdt::gdt_init, idt::interrupt};

pub fn x86_init() {
    interrupt::disable_interrupts();
    gdt_init();
}
