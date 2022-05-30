
//
// CPU abstraction of aarch64 CPU
//

use core::arch::asm;

#[inline(always)]
pub fn wait_forever() -> !
{
    unsafe {
        loop {
            asm!("wfe");
        }
    }
}
