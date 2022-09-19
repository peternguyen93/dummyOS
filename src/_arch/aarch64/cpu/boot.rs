//
// Boot Loader of AARCH64
//

use core::arch::global_asm;

// load boot.S
global_asm!(
    include_str!("boot.S"),
    CONST_CORE_ID_MASK = const 0b11
);

// Rust entry point
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init();
}
