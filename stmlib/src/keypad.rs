//! A module that adds a basic api for the Keypad.

use crate::gpio::GPIO;
use crate::PERIPHERALS;

/// Initializes the keypad
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

/// Waits until a key is pressed an returns the keycode.
pub unsafe fn keypad_getkey() -> u32 {
    0
}

/// Returns the keycode of a key if one is pressed.
/// If one is not pressed returns -1.
pub unsafe fn keypad_getkey_nb() -> i8 {
    -1
}

/// Waits untill a key is pressed and returns a char of the key
pub unsafe fn keypad_getchar() -> char {
    ' '
}
