mod gpioa;
mod gpiob;
mod gpioc;

use core::ptr::{read, write};

pub use gpioa::GPIOA;
pub use gpiob::GPIOB;
pub use gpioc::GPIOC;

/// Global trait that defines what a GPIO bus can do.
pub trait GPIO {
    /// Address to the moder register
    const MODER: *mut u32;
    /// Address to the otyper register
    const OTYPER: *mut u32;
    /// Address to the ospeedr register
    const OSPEEDR: *mut u32;
    /// Address to the pupdr register
    const PUPDR: *mut u32;
    /// Address to the idr register
    const IDR: *mut u32;
    /// Address to the odr register
    const ODR: *mut u32;
    /// Address to the bsrr register
    const BSRR: *mut u32;
    /// Address to the lckr register
    const LCKR: *mut u32;
    /// Address to the afrl register
    const AFRL: *mut u32;
    /// Address to the afrh register
    const AFRH: *mut u32;

    /// Sets bits in the MODER. Follows RMW.
    unsafe fn set_moder_bits(&mut self, bits: u32) {
        let mask = read(Self::MODER) | bits;
        write(Self::MODER, mask);
    }

    /// Clears bits in the MODER. Follows RMW.
    unsafe fn clear_moder_bits(&mut self, bits: u32) {
        let mask = read(Self::MODER) & !bits;
        write(Self::MODER, mask);
    }

    /// Returns the current value of the ODR.
    unsafe fn read_odr(&self) -> u32 {
        read(Self::ODR)
    }

    /// Clears bits in the ODR using the BSRR.
    unsafe fn clear_odr_bits(&mut self, val: u16) {
        let mask = (val as u32) << 16;
        write(Self::BSRR, mask);
    }

    /// Sets bits in the ODR using the BSRR.
    unsafe fn set_odr_bits(&mut self, val: u16) {
        write(Self::BSRR, val as u32);
    }

    /// Sets and clears bits in the ODR in one operation.
    unsafe fn set_clear_odr(&mut self, clear: u16, set: u16) {
        let mask = ((clear as u32) << 16) | set as u32;
        write(Self::BSRR, mask);
    }

    /// Returns the current vlaue of the PUPDR
    unsafe fn get_pupdr(&self) -> u32 {
        read(Self::PUPDR)
    }

    /// Sets bits in the PUPDR. Follows RMW.
    unsafe fn set_pupdr_bits(&mut self, bits: u32) {
        let mask = read(Self::PUPDR) | bits;
        write(Self::PUPDR, mask);
    }

    unsafe fn clear_pupdr_bits(&mut self, bits: u32) {
        let mask = read(Self::PUPDR) & !bits;
        write(Self::PUPDR, mask);
    }
}
