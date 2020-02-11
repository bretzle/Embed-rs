use crate::gpio::GPIO;
use crate::PERIPHERALS;

/// Represents GPIOB
#[repr(C)]
pub struct GPIOB;

// impl GPIO for GPIOB {
//     const MODER:   *mut u32 = 0x4002_0400 as *mut u32;
//     const OTYPER:  *mut u32 = 0x4002_0404 as *mut u32;
//     const OSPEEDR: *mut u32 = 0x4002_0408 as *mut u32;
//     const PUPDR:   *mut u32 = 0x4002_040C as *mut u32;
//     const IDR:     *mut u32 = 0x4002_0410 as *mut u32;
//     const ODR:     *mut u32 = 0x4002_0414 as *mut u32;
//     const BSRR:    *mut u32 = 0x4002_0418 as *mut u32;
//     const LCKR:    *mut u32 = 0x4002_041C as *mut u32;
//     const AFRL:    *mut u32 = 0x4002_0420 as *mut u32;
//     const AFRH:    *mut u32 = 0x4002_0424 as *mut u32;
// }

// impl Drop for GPIOB {
//     fn drop(&mut self) {
//         unsafe { PERIPHERALS.gpiob = Some(GPIOB) }
//     }
// }
