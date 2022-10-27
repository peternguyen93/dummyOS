/*
 * Main Kernel Implementation
 *
 */

#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod console;
mod print;
mod panic;
mod synchronization;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    loop {
        println!("Starting Dummy OS...");
        panic!("Halt");
    }
}
