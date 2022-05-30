/*
 * Main Kernel Implementation
 *
 */

#![no_std]
#![no_main]

mod bsp;
mod cpu;
mod panic;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    loop {
        
    }
}
