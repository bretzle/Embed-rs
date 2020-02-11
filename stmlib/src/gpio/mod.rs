//! Module that adds support for manipulating the GPIO busses

// mod gpioa;
// mod gpiob;
// mod gpioc;

use core::ptr::{read, write};

// pub use gpioa::GPIOA;
// pub use gpiob::GPIOB;
// pub use gpioc::GPIOC;

/// Global trait that defines what a GPIO bus can do.
pub struct GPIO {
    /// Address to the moder register
    pub moder: *mut u32,
    /// Address to the otyper register
    pub otyper: *mut u32,
    /// Address to the ospeedr register
    pub ospeedr: *mut u32,
    /// Address to the pupdr register
    pub pupdr: *mut u32,
    /// Address to the idr register
    pub idr: *mut u32,
    /// Address to the odr register
    pub odr: *mut u32,
    /// Address to the bsrr register
    pub bsrr: *mut u32,
    /// Address to the lckr register
    pub lckr: *mut u32,
    /// Address to the afrl register
    pub afrl: *mut u32,
    /// Address to the afrh register
    pub afrh: *mut u32,
}

impl GPIO {
    /// Sets bits in the MODER. Follows RMW.
    pub unsafe fn set_moder_bits(&mut self, bits: u32) {
        let mask = read(self.moder) | bits;
        write(self.moder, mask);
    }

    /// Clears bits in the MODER. Follows RMW.
    pub unsafe fn clear_moder_bits(&mut self, bits: u32) {
        let mask = read(self.moder) & !bits;
        write(self.moder, mask);
    }

    /// Returns the current value of the ODR.
    pub unsafe fn read_odr(&self) -> u32 {
        read(self.odr) & 0xFFFF
    }

    /// Clears bits in the ODR using the BSRR.
    pub unsafe fn clear_odr_bits(&mut self, val: u16) {
        let mask = (val as u32) << 16;
        write(self.bsrr, mask);
    }

    /// Sets bits in the ODR using the BSRR.
    pub unsafe fn set_odr_bits(&mut self, val: u16) {
        write(self.bsrr, val as u32);
    }

    /// Sets and clears bits in the ODR in one operation.
    pub unsafe fn set_clear_odr(&mut self, clear: u16, set: u16) {
        let mask = ((clear as u32) << 16) | set as u32;
        write(self.bsrr, mask);
    }

    /// Returns the current vlaue of the PUPDR
    pub unsafe fn get_pupdr(&self) -> u32 {
        read(self.pupdr)
    }

    /// Sets bits in the PUPDR. Follows RMW.
    pub unsafe fn set_pupdr_bits(&mut self, bits: u32) {
        let mask = read(self.pupdr) | bits;
        write(self.pupdr, mask);
    }

    /// Clears bits in the PUPDR. Follows RMW.
    pub unsafe fn clear_pupdr_bits(&mut self, bits: u32) {
        let mask = read(self.pupdr) & !bits;
        write(self.pupdr, mask);
    }

    /// Returns the current value of the IDR.
    pub unsafe fn get_idr(&self) -> u32 {
        read(self.idr) & 0xFFFF
    }
}

impl Copy for GPIO {}

impl Clone for GPIO {
    #[inline(always)]
    fn clone(&self) -> GPIO {
        *self
    }
}
