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

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn nmi_handler();
    fn hard_fault_handler();
    fn memory_management_handler();
    fn bus_fault_handler();
    fn usage_fault_handler();
    fn sv_call_handler();
    fn pending_sv_handler();
    fn systick_handler();
}

#[link_section = ".vector_table.exceptions"]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector {
        handler: nmi_handler,
    },
    Vector {
        handler: hard_fault_handler,
    },
    Vector {
        handler: memory_management_handler,
    },
    Vector {
        handler: bus_fault_handler,
    },
    Vector {
        handler: usage_fault_handler,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: sv_call_handler,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: pending_sv_handler,
    },
    Vector {
        handler: systick_handler,
    },
];

#[no_mangle]
pub fn default_exception_handler() {
    loop {}
}
