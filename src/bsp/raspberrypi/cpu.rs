
//
// CPU Public declaration for Raspberry Pi Board
//

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
