#![no_std]

pub mod gpio;
pub mod lcd;
pub mod led;
pub mod timer;

use crate::gpio::*;
use crate::timer::*;
use core::mem::replace;
use core::ptr;

pub const FREQ: u32 = 16_000_000;

pub struct Peripherals {
    systick: Option<SysTick>,
    gpioa: Option<GPIOA>,
    gpiob: Option<GPIOB>,
    gpioc: Option<GPIOC>,
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

    pub fn take_gpioa(&mut self) -> GPIOA {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x1) }
        let p = replace(&mut self.gpioa, None);
        p.unwrap()
    }

    pub fn take_gpioc(&mut self) -> GPIOC {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x4) }
        let p = replace(&mut self.gpioc, None);
        p.unwrap()
    }
}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    systick: Some(SysTick),
    gpioa: Some(GPIOA),
    gpiob: Some(GPIOB),
    gpioc: Some(GPIOC),
};
