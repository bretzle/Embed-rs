//! Module that adds support for utilizing the systems timers

mod systick;

use crate::PERIPHERALS;
pub use systick::SysTick;

/// Helper method to easily delay [time] milliseconds using SysTick
pub fn delay_ms(time: u32) {
    unsafe {
        let stk = PERIPHERALS.take_systick();
        stk.delay_ms(time);
    }
}

/// Helper method to easily delay [time] microseconds using SysTick
pub fn delay_us(time: u32) {
    unsafe {
        let stk = PERIPHERALS.take_systick();
        stk.delay_us(time);
    }
}