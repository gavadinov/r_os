#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use r_os::{serial_print, serial_println, println};
use test_macro::ros_test;

//noinspection RsUnresolvedReference
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    r_os::test_panic_handler(info)
}

#[ros_test]
fn test_println() {
    println!("TEST");
}
