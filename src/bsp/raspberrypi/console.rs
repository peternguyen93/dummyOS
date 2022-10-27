//
// Real Console output implementation
//

use crate::synchronization::SpinLock;
use crate::synchronization::interface::Mutex;
use core::fmt::{self, Write, Result};

struct QEMUOutput;

// serial address for output characters in qemu-system-aarch64
const QEMU_SERIAL_ADDRESS: u64 = 0x3F20_1000;

impl fmt::Write for QEMUOutput {
    // implement Rust traits fmt::Write to QEMUOutput structure
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        unsafe {
            // write characters directly to this mapped address
            core::ptr::write_volatile(QEMU_SERIAL_ADDRESS as *mut u8, c as u8);
        }

        Ok(())
    }

}

pub struct SpinLockQemuOutput {
    qemu_output: SpinLock<QEMUOutput>
}

impl SpinLockQemuOutput {
    pub const fn new() -> SpinLockQemuOutput {
        return SpinLockQemuOutput { qemu_output: SpinLock::new(QEMUOutput {}) };
    }

    pub fn write_fmt(&self, args: fmt::Arguments<'_>) -> Result {
        return self.qemu_output.lock(|qemu_output| {
            // protected this call from race condition
            return qemu_output.write_fmt(args);
        });
    }
}

static GUARDED_QEMU_OUTPUT: SpinLockQemuOutput = SpinLockQemuOutput::new();

// Exported Code
// Return a reference to the console.
pub fn console() -> &'static SpinLockQemuOutput {
    return &GUARDED_QEMU_OUTPUT;
}
