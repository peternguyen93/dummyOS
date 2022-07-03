//
// Panic Handler
//

use crate::cpu;
use crate::print;

use core::panic::PanicInfo;

/*
 *  Stop immediately if called a second time.
 *
 *  Using atomics here relieves us from needing to use `unsafe` for the static variable.
 *  On `AArch64`, which is the only implemented architecture at the time of writing this,
 *  [`AtomicBool::load`] and [`AtomicBool::store`] are lowered to ordinary load and store
 *  instructions. They are therefore safe to use even with MMU + caching deactivated.
 *  
 *  [`AtomicBool::load`]: core::sync::atomic::AtomicBool::load
 *  [`AtomicBool::store`]: core::sync::atomic::AtomicBool::store
 */
fn panic_prevent_reenter()
{

}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
    loop {
        cpu::wait_forever();
    }
}
