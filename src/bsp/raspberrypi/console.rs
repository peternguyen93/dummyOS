//
// Real Console output implementation
//

use crate::console;
use core::fmt;

struct QEMUOutput;

// serial address for output characters in qemu-system-aarch64
const QEMU_SERIAL_ADDRESS: u64 = 0x3F20_1000;
impl fmt::Write for QEMUOutput {
    // implement Rust traits fmt::Write to QEMUOutput structure
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        unsafe {
            // write characters directly to this mapped address
            core::ptr::write_volatile(QEMU_SERIAL_ADDRESS as *mut u8, c as u8);
        }

        Ok(())
    }
}

// Exported Code
// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}
