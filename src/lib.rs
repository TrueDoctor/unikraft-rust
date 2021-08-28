#![feature(alloc_error_handler)]
#![no_std]

use core::panic::PanicInfo;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn hello_rust() {
    unsafe { libc::printf("hallo\n\n".as_ptr() as *const i8) };
    /*let a = vec![3, 4, 2, 1, 4, 2, 7];
    let b: Vec<_> = a.iter().map(|x| x + x).collect();
    for n in b {
        println!("HI {}", n);
    }*/
}
#[panic_handler]
fn panic_rust(_: &PanicInfo) -> ! {
    loop {}
}
