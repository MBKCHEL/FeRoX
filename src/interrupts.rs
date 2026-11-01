use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::kprint;
use lazy_static::lazy_static;
use core::arch::asm;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    kprint!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> !{
    kprint!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {
        unsafe { asm!("hlt") };
    }
}

