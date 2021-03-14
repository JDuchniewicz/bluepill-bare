//! Very rudimentary bare metal driver for Bluepill
//!
//! DO NOT use it for anything else than this application. Use `stm32f1xx-hal` instead
#![allow(dead_code)]

pub mod gpio {
    use core::{
        ops::{BitAndAssign, BitOrAssign},
        sync::atomic::{AtomicBool, Ordering::SeqCst},
    };

    pub struct Pin(u8);

    pub struct Pins {
        pub p_c_13: Pin,
    }

    impl Pins {
        pub fn take() -> Self {
            static TAKEN: AtomicBool = AtomicBool::new(false);

            // enforce this is a singleton
            assert!(!TAKEN.swap(true, SeqCst));

            Self { p_c_13: Pin(13) }
        }
    }

    #[derive(Copy, Clone)]
    pub enum Level {
        Low,
        High,
    }

    const GPIOC_BASE: u32 = 0x4001_1000;

    const GPIOC_CRH: *mut u32 = (GPIOC_BASE + 0x04) as *mut u32;
    const GPIOC_ODR: *mut u32 = (GPIOC_BASE + 0x0C) as *mut u32;

    impl Pin {
        // configure GPIO C as push-pull output
        pub fn set_push_pull_output(&mut self, level: Level) {
            // set up GPIOC_CRH as CNF=0b00 (output push-pull) and MODE=0b10 (output, low speed)
            unsafe { core::ptr::write_volatile(GPIOC_CRH, 0x00200000) };

            match level {
                Level::High => self.set_high(),
                Level::Low => self.set_low(),
            }
        }

        pub fn set_high(&mut self) {
            unsafe {
                let mut mask = core::ptr::read_volatile(GPIOC_ODR);
                mask.bitor_assign(1 << (self.0 as u32));
                core::ptr::write_volatile(GPIOC_ODR, mask);
            }
        }

        pub fn set_low(&mut self) {
            unsafe {
                let mut mask = core::ptr::read_volatile(GPIOC_ODR);
                mask.bitand_assign(1 << (self.0 as u32));
                core::ptr::write_volatile(GPIOC_ODR, mask);
            }
        }
    }
}

pub mod rcc {
    const RCC_BASE: u32 = 0x4002_1000;

    const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x18) as *mut u32;

    const RCC_IOPCEN: u32 = 1 << 4;

    // turn on the clock for GPIOC
    pub fn apb2_enable_register() {
        unsafe { core::ptr::write_volatile(RCC_APB2ENR, RCC_IOPCEN) }
    }
}
