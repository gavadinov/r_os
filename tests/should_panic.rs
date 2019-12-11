#![no_std]
#![no_main]

use core::panic::PanicInfo;
use r_os::{QemuExitCode, exit_qemu, serial_println, serial_print};

//noinspection RsUnresolvedReference
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}
