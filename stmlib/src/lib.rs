#![no_std]

pub mod timer;

use crate::timer::SysTick;
use core::mem::replace;

pub const FREQ: u32 = 16_000_000;

pub struct Peripherals {
    systick: Option<SysTick>,
}

impl Peripherals {
    pub fn take_systick(&mut self) -> SysTick {
        let p = replace(&mut self.systick, None);
        p.unwrap()
    }
}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    systick: Some(SysTick),
};
