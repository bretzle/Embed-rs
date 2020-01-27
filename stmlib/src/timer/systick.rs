use crate::FREQ;
use crate::PERIPHERALS;
use core::ptr;

/// Represents the System Timer
#[repr(C)]
pub struct SysTick;

impl SysTick {
    const CTRL: *mut u32 = 0xE000_E010 as *mut u32;
    const LOAD: *mut u32 = 0xE000_E014 as *mut u32;
    // const VAL: *mut u32 = 0xE000_E018 as *mut u32;
    // const CALIB: *mut u32 = 0xE000_E01C as *mut u32;

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

    /// Blocks the system from doing anything for [delay] milliseconds
    pub fn delay_ms(&self, delay: u32) {
        self.delay_us(delay * 1000);
    }

    /// Blocks the system from doing anything for [delay] microseconds
    pub fn delay_us(&self, delay: u32) {
        unsafe {
            self.disable();
            self.set_load(delay * (FREQ / 8_000_000));
            self.enable();

            while !self.get_count_flag() {}

            self.disable();
        }
    }
}

impl Drop for SysTick {
    fn drop(&mut self) {
        unsafe { PERIPHERALS.systick = Some(SysTick) }
    }
}
