#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                         // extern crate panic_abort; // requires nightly
                         // extern crate panic_itm; // logs messages over ITM; requires ITM support
                         // extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::mem::replace;
use core::ptr;
use cortex_m_rt as rt;

pub const FREQ: u32 = 16_000_000;

struct Peripherals {
    systick: Option<SysTick>,
}
impl Peripherals {
    fn take_systick(&mut self) -> SysTick {
        let p = replace(&mut self.systick, None);
        p.unwrap()
    }
}
static mut PERIPHERALS: Peripherals = Peripherals {
    systick: Some(SysTick),
};

#[repr(C)]
pub struct SysTick;

impl SysTick {
    const CTRL: *mut u32 = 0xE000_E010 as *mut u32;
    const LOAD: *mut u32 = 0xE000_E014 as *mut u32;
    const VAL: *mut u32 = 0xE000_E018 as *mut u32;
    const CALIB: *mut u32 = 0xE000_E01C as *mut u32;

    #[inline(always)]
    unsafe fn disable(&self) {
        ptr::write(Self::CTRL, 0);
    }

    #[inline(always)]
    unsafe fn enable(&self) {
        ptr::write(Self::CTRL, 1);
    }

    #[inline(always)]
    unsafe fn set_load(&self, val: u32) {
        ptr::write(Self::LOAD, val);
    }

    #[inline(always)]
    unsafe fn get_count_flag(&self) -> bool {
        let reg = (ptr::read(Self::CTRL) >> 16) & 0b1;
        if reg == 1 {
            return true;
        }
        false
    }

    pub fn delay_ms(&self, delay: u32) {
        unsafe {
            self.disable();
            self.set_load(delay * (FREQ / 8000));
            self.enable();

            while !self.get_count_flag() {}

            self.disable();
        }
    }

    pub fn delay_us(&self, delay: u32) {}
}

impl Drop for SysTick {
    fn drop(&mut self) {
        unsafe { PERIPHERALS.systick = Some(SysTick) }
    }
}

#[rt::entry]
fn main() -> ! {
    unsafe {
        {
            let a: SysTick = PERIPHERALS.take_systick();
            a.delay_ms(5000);
        }
        let b = PERIPHERALS.take_systick();
        core::mem::drop(b);
        let c = PERIPHERALS.take_systick();
        loop {}
    }
}
