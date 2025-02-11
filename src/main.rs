#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(milly_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    milly_os::init();

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    milly_os::hlt_loop();
}
