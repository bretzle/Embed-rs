use crate::gpio::GPIOC;
use crate::PERIPHERALS;

pub unsafe fn init_keypad() {
    let mut gpio: GPIOC = PERIPHERALS.take_gpioc();

    gpio.set_moder(0x55);

    let mut val = gpio.get_pupdr();

    gpio.set_pupdr(0x1);

    val = gpio.get_pupdr();

    gpio.set_pupdr(0x10);

    val = gpio.get_pupdr();

    let a = 5;
}

pub unsafe fn keypad_getkey() -> u32 {
    0
}

pub unsafe fn keypad_getkey_nb() -> i8 {
    -1
}

pub unsafe fn keypad_getchar() -> char {
    ' '
}
