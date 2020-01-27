#![no_std]
#![no_main]
#![allow(dead_code)]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt as rt;
use stmlib::led::*;
use stmlib::*;

#[rt::entry]
fn main() -> ! {
    unsafe {
        // knight_rider();
        // basic_lcd();
        lcd_key();
        loop {}
    }
}

unsafe fn knight_rider() {
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

unsafe fn basic_lcd() {
    lcd::init_lcd();
    lcd::lcd_set_postion(1, 5);
    lcd::lcd_print_string("hello");
    lcd::lcd_home();
    lcd::lcd_print_num(0xFFFF_FFFF);
}

unsafe fn lcd_key() {
    lcd::init_lcd();
    keypad::init_keypad();

    lcd::lcd_clear();

    loop {
        let letter = keypad::keypad_getchar();

        match letter {
            '*' => lcd::lcd_clear(),
            '#' => lcd::lcd_home(),
            _ => lcd::lcd_print_char(letter),
        }
    }
}
