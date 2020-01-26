#![no_std]

pub mod gpio;
pub mod timer;

use crate::gpio::*;
use crate::timer::*;
use core::mem::replace;
use core::ptr;

pub const FREQ: u32 = 16_000_000;

pub struct Peripherals {
    systick: Option<SysTick>,
    gpiob: Option<GPIOB>,
}

impl Peripherals {
    const RCC: *mut u32 = 0x4002_3830 as *mut u32;

    pub fn take_systick(&mut self) -> SysTick {
        let p = replace(&mut self.systick, None);
        p.unwrap()
    }

    pub fn take_gpiob(&mut self) -> GPIOB {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x2) }
        let p = replace(&mut self.gpiob, None);
        p.unwrap()
    }
}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    systick: Some(SysTick),
    gpiob: Some(GPIOB),
};
