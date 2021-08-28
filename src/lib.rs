#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]
#![no_std]

use core::panic::PanicInfo;
extern crate alloc;
use alloc::vec::Vec;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn hello_rust() {
    print("Hello rust\n\0");
    let a = alloc::vec![3, 4, 2, 1, 4, 2, 7];
    let b: Vec<_> = a.iter().map(|x| x + x).collect();
    for n in b {
        print("HI from loop\n\0");
    }
}

fn print(msg: &str) {
    unsafe { libc::printf(msg.as_ptr() as *const i8) };
}
#[panic_handler]
fn panic_rust(_: &PanicInfo) -> ! {
    loop {}
}
