#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt as rt;

pub const FREQ: u32 = 16_000_000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysTick {
    ctrl: u32,
    load: u32,
    val: u32,
    calib: u32,
}

impl SysTick {
    pub fn get() -> SysTick {
        unsafe { *(0xE000_E010 as *const SysTick) }
    }
}

#[rt::entry]
fn main() -> ! {
    unsafe {
        let a = SysTick::get();
        let b = SysTick::get();
        loop {}
    }
}
