#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Utopac OS demarrer");
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Demarrage de {} tests", tests.len());
    for test in tests {
        test();
    }
}