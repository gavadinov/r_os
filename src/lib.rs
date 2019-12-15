#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crate::interrupts::init_idt;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;

pub fn init() {
    init_idt();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!();
    serial_println!();
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("\x1B[31m ... [failed]\x1B[0m");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

//noinspection RsUnresolvedReference
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
