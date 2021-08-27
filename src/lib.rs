#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn hello_rust() {
    printf("Hi!\n\0".as_ptr());
}

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_fmt: &PanicInfo<'_>) -> ! {
    unsafe { printf("Rust panic\0\n".as_ptr()) };
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
