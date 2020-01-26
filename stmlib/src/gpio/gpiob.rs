use crate::PERIPHERALS;
use core::ptr;

#[repr(C)]
pub struct GPIOB;

impl GPIOB {
    const MODER: *mut u32 = 0x4002_0400 as *mut u32;
    const IDR: *mut u32 = 0x4002_0410 as *mut u32;
    const ODR: *mut u32 = 0x4002_0414 as *mut u32;

    pub unsafe fn set_moder(&mut self, mask: u32) {
        let val = ptr::read(Self::MODER) | mask;
        ptr::write(Self::MODER, val);
    }

    pub unsafe fn clear_odr(&mut self) {
        ptr::write(Self::ODR, 0);
    }

    pub unsafe fn set_odr(&mut self, val: u32) {
        ptr::write(Self::ODR, val);
    }
}

impl Drop for GPIOB {
    fn drop(&mut self) {
        unsafe { PERIPHERALS.gpiob = Some(GPIOB) }
    }
}
