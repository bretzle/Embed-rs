//! A module that adds a basic api for the LED array.

use crate::PERIPHERALS;

/// Initializes the LED array
pub fn init_leds() {
    unsafe {
        let mut gpio = PERIPHERALS.take_gpiob();

        gpio.set_moder_bits(0x55155400);
    }
}

/// Displays a binary number on the LED array.
pub fn light_led(num: u16) {
    unsafe {
        let mut gpio = PERIPHERALS.take_gpiob();

        let mask = ((num & 0x3F) << 5) | ((num & 0x3C0) << 6);

        gpio.set_clear_odr(0xF7E0, mask);
    }
}
