#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
use blog_os_diy::println;
use blog_os_diy::print;



/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os_diy::gdt::init_gdt();
    blog_os_diy::interrupts::init_idt();
    unsafe { blog_os_diy::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    //trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    println!("It did not crash!");
    blog_os_diy::hlt_loop(); 
}



/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os_diy::hlt_loop();
}