#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt as rt;
use stmlib::*;

#[rt::entry]
fn main() -> ! {
    unsafe {
        {
            let a: SysTick = PERIPHERALS.take_systick();
            a.delay_ms(5000);
        }
        let b = PERIPHERALS.take_systick();
        core::mem::drop(b);
        let c = PERIPHERALS.take_systick();
        loop {}
    }
}
