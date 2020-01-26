use crate::gpio::*;
use crate::PERIPHERALS;
use core::mem::drop;
use numtoa::NumToA;

const RS_SET: u16 = 1 << 8;
const RW_SET: u16 = 1 << 9;
const E_SET: u16 = 1 << 10;

pub unsafe fn init_lcd() {
    let mut a: GPIOA = PERIPHERALS.take_gpioa();
    let mut c: GPIOC = PERIPHERALS.take_gpioc();

    a.set_moder(0x555500);
    c.set_moder(0x150000);

    drop(a);
    drop(c);

    let stk = PERIPHERALS.take_systick();
    stk.delay_ms(40);
    drop(stk);

    lcd_cmd_d(0x38, 37);
    lcd_cmd_d(0x38, 37);
    lcd_cmd_d(0x0F, 37);
    lcd_cmd_d(0x01, 1520);
    lcd_cmd_d(0x06, 37);
}

unsafe fn lcd_cmd_d(instruction: u8, delay: u32) {
    let mut a: GPIOA = PERIPHERALS.take_gpioa();
    let mut c: GPIOC = PERIPHERALS.take_gpioc();
    let stk = PERIPHERALS.take_systick();

    c.set_clear_odr(RS_SET | RW_SET, E_SET);

    a.clear_odr_bits(0xFF0);
    a.set_odr_bits((instruction as u16) << 4);

    c.clear_odr_bits(E_SET);

    stk.delay_us(delay);
}

unsafe fn lcd_data(data: u8) {
    let mut a: GPIOA = PERIPHERALS.take_gpioa();
    let mut c: GPIOC = PERIPHERALS.take_gpioc();
    let stk = PERIPHERALS.take_systick();

    c.set_clear_odr(RW_SET, RS_SET | E_SET);

    a.clear_odr_bits(0xFF0);
    a.set_odr_bits((data as u16) << 4);

    c.clear_odr_bits(E_SET);

    stk.delay_ms(2);
}

pub unsafe fn lcd_clear() {
    lcd_cmd_d(0x01, 1520);
}

pub unsafe fn lcd_home() {
    lcd_cmd_d(0x02, 1520);
}

pub unsafe fn lcd_set_postion(row: u8, col: u8) {
    let cmd = (1 << 7) + (col + (row * 0x40));
    lcd_cmd_d(cmd, 37);
}

pub unsafe fn lcd_print_string(string: &str) {
    for ch in string.bytes() {
        lcd_data(ch);
    }
}

pub unsafe fn lcd_print_num(num: u32) {
    let mut buffer = [0u8; 10];
    lcd_print_string(num.numtoa_str(10, &mut buffer));
}
