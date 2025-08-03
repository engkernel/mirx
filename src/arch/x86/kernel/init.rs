use crate::arch::x86::{gdt::gdt::gdt_init, idt::{idt::idt_init, interrupt}};

pub fn x86_init() {
    interrupt::disable_interrupts();
    gdt_init();
    idt_init();
    interrupt::enable_interrupts();
}
