use crate::gpio::GPIO;
use crate::PERIPHERALS;

/// Represents GPIOA
#[repr(C)]
pub struct GPIOA;

// impl GPIO for GPIOA {
//     const MODER:   *mut u32 = 0x4002_0000 as *mut u32;
//     const OTYPER:  *mut u32 = 0x4002_0004 as *mut u32;
//     const OSPEEDR: *mut u32 = 0x4002_0008 as *mut u32;
//     const PUPDR:   *mut u32 = 0x4002_000C as *mut u32;
//     const IDR:     *mut u32 = 0x4002_0010 as *mut u32;
//     const ODR:     *mut u32 = 0x4002_0014 as *mut u32;
//     const BSRR:    *mut u32 = 0x4002_0018 as *mut u32;
//     const LCKR:    *mut u32 = 0x4002_001C as *mut u32;
//     const AFRL:    *mut u32 = 0x4002_0020 as *mut u32;
//     const AFRH:    *mut u32 = 0x4002_0024 as *mut u32;
// }

// impl Drop for GPIOA {
//     fn drop(&mut self) {
//         unsafe { PERIPHERALS.gpioa = Some(GPIOA) }
//     }
// }
