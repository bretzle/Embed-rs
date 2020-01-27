use crate::gpio::GPIO;
use crate::PERIPHERALS;

pub unsafe fn init_keypad() {
    let mut gpio = PERIPHERALS.take_gpioc();

    gpio.set_moder_bits(0x55);

    let mut val = gpio.get_pupdr();

    gpio.set_pupdr_bits(0x1);

    val = gpio.get_pupdr();

    gpio.set_pupdr_bits(0x10);

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
