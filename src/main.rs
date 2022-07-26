#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod libs;

use core::panic::PanicInfo;
use crate::libs::{exit_qemu, hlt_loop, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Utopac OS demarrer");
    libs::init();


    hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    println!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    hlt_loop();
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Demarrage de {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}