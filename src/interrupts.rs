use crate::gdt;
use crate::println;
#[cfg(test)]
use crate::{serial_print, serial_println};
use lazy_static::lazy_static;
use test_macro::ros_test;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    println!("DOUBLE FAULT");
    println!("{:#?}", stack_frame);
    panic!();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("BREAKPOINT EXCEPTION");
    println!("{:#?}", stack_frame);
}

#[ros_test]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
