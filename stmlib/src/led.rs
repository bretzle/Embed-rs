use crate::GPIOB;
use crate::PERIPHERALS;

pub unsafe fn init_leds() {
    let mut gpio: GPIOB = PERIPHERALS.take_gpiob();

    gpio.set_moder(0x55155400);
}

pub unsafe fn light_led(num: u16) {
    let mut gpio: GPIOB = PERIPHERALS.take_gpiob();
    const clear_mask: u16 = 0xF7E0;
    let set_mask = ((num & 0x3F) << 5) | ((num & 0x3C0) << 6);

    gpio.set_clear_odr(clear_mask, set_mask);
}
