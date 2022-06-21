//
// Panic Handler
//

use crate::cpu;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
    loop {
        cpu::wait_forever();
    }
}
