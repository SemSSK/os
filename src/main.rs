#![no_std] //removes the usage of the standard library
#![no_main] // tells the compiler to not look for a main function as an entry point
use core::panic::PanicInfo;

// custom panic handler as the standard library isn't available
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}

// the start function is the entry point for the bootloader
#[no_mangle] // no mangle stops the compiler from changing the name of the function
pub extern "C" fn _start() -> ! { // extern c means that the function calls on the c runtime
    loop {}
}
