//
// Boot Loader of AARCH64
//

use core::arch::global_asm;

// load boot.S
global_asm!(include_str!("boot.S"));

// Rust entry point
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init();
}
