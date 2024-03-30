#![no_std]

use core::{mem, panic::PanicInfo, ptr};

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}

pub unsafe fn reset_handler() -> ! {
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;
        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
    }

    let mut bss_pointer = ptr::from_mut(&mut _sbss);
    let bss_end = ptr::from_mut(&mut _ebss);
    while bss_pointer < bss_end {
        ptr::write(bss_pointer, mem::zeroed());
        bss_pointer = bss_pointer.add(1);
    }

    let mut source_pointer = ptr::from_ref(&_sidata);
    let mut destination_pointer = ptr::from_mut(&mut _sdata);
    let data_end = ptr::from_mut(&mut _edata);
    while destination_pointer < data_end {
        ptr::write(destination_pointer, ptr::read(source_pointer));
        source_pointer = source_pointer.add(1);
        destination_pointer = destination_pointer.add(1);
    }

    extern "Rust" {
        fn main();
    }
    main();

    loop {}
}

#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe fn() -> ! = reset_handler;
