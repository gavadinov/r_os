#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use r_os::println;
use r_os::init;

//noinspection RsUnresolvedReference
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    println!("Asdf");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    r_os::test_panic_handler(info)
}
