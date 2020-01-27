use crate::PERIPHERALS;
use core::ptr;

#[repr(C)]
pub struct GPIOC;

impl GPIOC {
    const MODER: *mut u32 = 0x4002_0800 as *mut u32;
    const IDR:   *mut u32 = 0x4002_0810 as *mut u32;
    const ODR:   *mut u32 = 0x4002_0814 as *mut u32;
    const BSRR:  *mut u32 = 0x4002_0818 as *mut u32;
    const PUPDR: *mut u32 = 0x4002_080C as *mut u32;

    pub unsafe fn set_moder(&mut self, mask: u32) {
        let val = ptr::read(Self::MODER) | mask;
        ptr::write(Self::MODER, val);
    }

    pub unsafe fn read_odr(&self) -> u32 {
        ptr::read(Self::ODR)
    }

    pub unsafe fn clear_odr_bits(&mut self, val: u16) {
        let mask = (val as u32) << 16;
        ptr::write(Self::BSRR, mask);
    }

    pub unsafe fn set_odr_bits(&mut self, val: u16) {
        ptr::write(Self::BSRR, val as u32);
    }

    pub unsafe fn set_clear_odr(&mut self, clear: u16, set: u16) {
        let mask = ((clear as u32) << 16) | set as u32;
        ptr::write(Self::BSRR, mask);
    }

    pub unsafe fn set_pupdr(&mut self, set: u32) {
        ptr::write(Self::PUPDR, set);
    }

    pub unsafe fn get_pupdr(&self) -> u32 {
        ptr::read(Self::PUPDR)
    }
}

impl Drop for GPIOC {
    fn drop(&mut self) {
        unsafe { PERIPHERALS.gpioc = Some(GPIOC) }
    }
}