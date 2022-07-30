//
// Panic Handler
//
use crate::{cpu, println};
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
use core::sync::atomic::{AtomicBool, Ordering};

fn panic_prevent_reenter()
{
    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);
        return;
    }

    cpu::wait_forever();
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
    panic_prevent_reenter();

    // extract information
    let (location, line, column) = match _info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0 as u32, 0 as u32)
    };

    // print kernel panic message
    println!(
        "Kernel Panic!\n\n\
        Panic location:\n\
        File '{}', line {}, column {}\n\n\
        {}", location, line, column, _info.message().unwrap_or(&format_args!("NULL")));

    cpu::wait_forever();
}
