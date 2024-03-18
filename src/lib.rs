#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

pub fn reset_handler() -> ! {
    extern "Rust" {
        fn main();
    }
    unsafe {
        main();
    }
    loop {}
}

#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: fn() -> ! = reset_handler;
