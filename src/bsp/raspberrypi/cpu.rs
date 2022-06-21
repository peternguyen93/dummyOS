
//
// CPU Public declaration for raspberry pi Board
//

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static CONST_CORE_ID_MASK: u64 = 0b11;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
