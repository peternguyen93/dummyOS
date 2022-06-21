//
// Console Abstraction
//

use crate::bsp;

pub mod interface {
    pub use core::fmt::Write;
}

pub mod console {
}

pub fn console() -> impl interface::Write {
    bsp::console::console();
}

