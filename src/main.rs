#![no_std] //Don't link the standard library
#![no_main] // tells the compiler to not look for a main function as an entry point
use core::{panic::PanicInfo};

// custom panic handler as the standard library isn't available
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello,world!";

// the start function is the entry point for the bootloader
#[no_mangle] // no mangle stops the compiler from changing the name of the function
pub extern "C" fn _start() -> ! { // extern c means that the function calls on the c runtime
    let vga_buffer = 0xb8000 as *mut u8;
    for (i,&byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
