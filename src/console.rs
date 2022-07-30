//
// Console Abstraction
//

use crate::bsp;

pub mod interface {
    // use default Write class
    pub use core::fmt::Write;
}

pub fn console() -> impl interface::Write {
    // call the custom implement for interface::Write in bsp/raspberrypi/console.rs
    bsp::console::console()
}

