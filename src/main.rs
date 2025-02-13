#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(milly_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

mod serial;
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    milly_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    milly_os::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello world{}", "!");
    milly_os::init();

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    milly_os::hlt_loop();
}
