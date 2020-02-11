//! A module that adds a basic api for the LCD.

use crate::gpio::*;
use crate::timer;
use crate::PERIPHERALS;
use core::mem::drop;
use core::str::from_utf8;
use numtoa::NumToA;

const RS_SET: u16 = 1 << 8;
const RW_SET: u16 = 1 << 9;
const E_SET: u16 = 1 << 10;

/// Initializes the LCD
pub fn init_lcd() {
    unsafe {
        let mut a = PERIPHERALS.take_gpioa();
        let mut c = PERIPHERALS.take_gpioc();

        a.set_moder_bits(0x555500);
        c.set_moder_bits(0x150000);

        drop(a);
        drop(c);

        timer::delay_ms(40);

        lcd_cmd_d(0x38, 37);
        lcd_cmd_d(0x38, 37);
        lcd_cmd_d(0x0F, 37);
        lcd_cmd_d(0x01, 1520);
        lcd_cmd_d(0x06, 37);
    }
}

unsafe fn lcd_cmd_d(instruction: u8, delay: u32) {
    let mut a = PERIPHERALS.take_gpioa();
    let mut c = PERIPHERALS.take_gpioc();

    c.set_clear_odr(RS_SET | RW_SET, E_SET);

    a.clear_odr_bits(0xFF0);
    a.set_odr_bits((instruction as u16) << 4);

    c.clear_odr_bits(E_SET);

    timer::delay_us(delay);
}

unsafe fn lcd_data(data: u8) {
    let mut a = PERIPHERALS.take_gpioa();
    let mut c = PERIPHERALS.take_gpioc();

    c.set_clear_odr(RW_SET, RS_SET | E_SET);

    a.clear_odr_bits(0xFF0);
    a.set_odr_bits((data as u16) << 4);

    c.clear_odr_bits(E_SET);

    timer::delay_ms(2);
}

/// Clears the LCD display and moves the cursor home
pub fn lcd_clear() {
    unsafe {
        lcd_cmd_d(0x01, 1520);
    }
}

/// Moves the cursor back home
pub fn lcd_home() {
    unsafe {
        lcd_cmd_d(0x02, 1520);
    }
}

/// Moves the cursor to a given postion
pub fn lcd_set_postion(row: u8, col: u8) {
    let cmd = (1 << 7) + (col + (row * 0x40));
    unsafe {
        lcd_cmd_d(cmd, 37);
    }
}

/// Prints a string to the LCD
pub fn lcd_print_string(string: &str) {
    unsafe {
        for ch in string.bytes() {
            lcd_data(ch);
        }
    }
}

/// Prints a number to the LCD
pub fn lcd_print_num(num: u32) {
    let mut buffer = [0u8; 10];
    lcd_print_string(num.numtoa_str(10, &mut buffer));
}

/// Prints a char to the LCD
pub fn lcd_print_char(letter: char) {
    let mut buffer = [0u8; 1];
    buffer[0] = letter as u8;
    lcd_print_string(from_utf8(&buffer).unwrap_or("CharError"));
}
