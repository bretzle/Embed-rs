//! A module that adds a basic api for the LED array.

use crate::gpio::GPIO;
use crate::PERIPHERALS;

/// Initializes the LED array
pub unsafe fn init_leds() {
    let mut gpio = PERIPHERALS.take_gpiob();

    gpio.set_moder_bits(0x55155400);
}

/// Displays a binary number on the LED array.
pub unsafe fn light_led(num: u16) {
    let mut gpio = PERIPHERALS.take_gpiob();
    const clear_mask: u16 = 0xF7E0;
    let set_mask = ((num & 0x3F) << 5) | ((num & 0x3C0) << 6);

    gpio.set_clear_odr(clear_mask, set_mask);
}
