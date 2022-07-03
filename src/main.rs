/*
 * Main Kernel Implementation
 *
 */

#![no_std]
#![no_main]

mod bsp;
mod cpu;
mod console;
mod print;
mod panic;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    loop {
        println!("Dummy OS is booted");

        //panic::panic();
    }
}
