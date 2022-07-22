#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Utopac OS demarrer");

    #[cfg(test)]
    test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
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