#![no_std]
#![no_main]

use core::{
    mem::zeroed,
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: fn() -> ! = reset_handler;

pub fn reset_handler() -> ! {
    extern "C" {
        static mut __sbss: u32; // Start of .bss
        static mut __ebss: u32; // End of .bss
        static mut __sdata: u32; // Start of .data
        static mut __edata: u32; // End of .data
        static __sidata: u32; // Start of .rodata
    }

    // Zero-Initialize .bss section
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize data
    unsafe {
        let mut sdata: *mut u32 = &mut __sdata;
        let edata: *mut u32 = &mut __edata;
        let mut sidata: *const u32 = &__sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Call the main function
    main()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}

fn delay(ticks: usize) {
    static mut DUMMY: usize = 0;

    for t in 0..ticks {
        // unsafe for preventing optimizing it out
        unsafe { write_volatile(&mut DUMMY, t) }
    }
}

mod bluepill;
use bluepill::{
    gpio::{Level, Pins},
    rcc::apb2_enable_register,
};

fn main() -> ! {
    let gpios = Pins::take();
    let mut led = gpios.p_c_13;

    apb2_enable_register();
    led.set_push_pull_output(Level::Low);

    loop {
        led.set_high();
        delay(2_000_0);

        led.set_low();
        delay(6_000_0);
    }
}
