//! A hardware abstraction layer designed for the stm32f446

#![no_std]
#![deny(missing_docs)]

pub mod gpio;
pub mod keypad;
pub mod lcd;
pub mod led;
pub mod timer;

use crate::gpio::*;
use crate::timer::*;
use core::mem::replace;
use core::ptr;

/// Default clock frequency
pub const FREQ: u32 = 16_000_000;

/// Struct to hold all peripherals for the system.
/// Each peripheral in the struct is a singleton. When the peripheral is dropped
/// it is readded to the struct and can be taken by another actor.
pub struct Peripherals {
    systick: Option<SysTick>,
    gpioa: Option<GPIOA>,
    gpiob: Option<GPIOB>,
    gpioc: Option<GPIOC>,
}

impl Peripherals {
    const RCC: *mut u32 = 0x4002_3830 as *mut u32;

    /// Take ownership of the Systick.
    pub fn take_systick(&mut self) -> SysTick {
        let p = replace(&mut self.systick, None);
        p.unwrap()
    }

    /// Take ownership of GPIOB
    pub fn take_gpiob(&mut self) -> GPIOB {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x2) }
        let p = replace(&mut self.gpiob, None);
        p.unwrap()
    }

    /// Take ownership of GPIOA
    pub fn take_gpioa(&mut self) -> GPIOA {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x1) }
        let p = replace(&mut self.gpioa, None);
        p.unwrap()
    }

    /// Take ownership of GPIOC
    pub fn take_gpioc(&mut self) -> GPIOC {
        unsafe { ptr::write(Self::RCC, ptr::read(Self::RCC) | 0x4) }
        let p = replace(&mut self.gpioc, None);
        p.unwrap()
    }
}

/// A public instance of all system peripherals
pub static mut PERIPHERALS: Peripherals = Peripherals {
    systick: Some(SysTick),
    gpioa: Some(GPIOA),
    gpiob: Some(GPIOB),
    gpioc: Some(GPIOC),
};
