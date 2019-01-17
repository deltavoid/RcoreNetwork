#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![feature(abi_x86_interrupt)]


// NEW: We need to add `pub` here to make them accessible from the outside
pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod gdt;


pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}


pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


pub fn trigger_a_page_fault() {
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
}
