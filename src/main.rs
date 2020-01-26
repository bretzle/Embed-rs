#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt as rt;
use stmlib::gpio::GPIOB;
use stmlib::*;

#[rt::entry]
fn main() -> ! {
    unsafe {
        let mut num = 1;
        let delay = 100;
        let stk = PERIPHERALS.take_systick();

        init_leds();

        loop {
            for _ in 0..9 {
                light_led(num);
                stk.delay_ms(delay);
                num = num << 1;
            }
            for _ in 0..9 {
                light_led(num);
                stk.delay_ms(delay);
                num = num >> 1;
            }
        }
    }
}

unsafe fn init_leds() {
    let mut gpio: GPIOB = PERIPHERALS.take_gpiob();

    gpio.set_moder(0x55155400);
}

unsafe fn light_led(num: u32) {
    let mut gpio: GPIOB = PERIPHERALS.take_gpiob();

    gpio.clear_odr();

    let a = (num & 0x3F) << 5;
    let b = (num & 0x3C0) << 6;

    gpio.set_odr(a | b);
}
