use crate::PERIPHERALS;
use crate::gpio::GPIO;

/// Represents GPIOC
#[repr(C)]
pub struct GPIOC;

impl GPIO for GPIOC {
    const MODER:   *mut u32 = 0x4002_0800 as *mut u32;
    const OTYPER:  *mut u32 = 0x4002_0804 as *mut u32;
    const OSPEEDR: *mut u32 = 0x4002_0808 as *mut u32;
    const PUPDR:   *mut u32 = 0x4002_080C as *mut u32;
    const IDR:     *mut u32 = 0x4002_0810 as *mut u32;
    const ODR:     *mut u32 = 0x4002_0814 as *mut u32;
    const BSRR:    *mut u32 = 0x4002_0818 as *mut u32;
    const LCKR:    *mut u32 = 0x4002_081C as *mut u32;
    const AFRL:    *mut u32 = 0x4002_0820 as *mut u32;
    const AFRH:    *mut u32 = 0x4002_0824 as *mut u32;
}

impl Drop for GPIOC {
    fn drop(&mut self) {
        unsafe { PERIPHERALS.gpioc = Some(GPIOC) }
    }
}