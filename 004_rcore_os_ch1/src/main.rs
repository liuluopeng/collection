// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]



use core::arch::global_asm;


#[macro_use]
mod console;
mod lang_items;
mod sbi;


global_asm!(include_str!("./entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    // loop {}
    clear_bss();
    println!("Hel发lo, world!");
    // sbi::shutdown()
    panic!("Shutdown machine!");

}


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    } 
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}