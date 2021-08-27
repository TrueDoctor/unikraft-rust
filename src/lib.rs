//use core::panic::PanicInfo;
#![feature(restricted_std, alloc_error_handler)]

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    let heap_start = 0x10000000;
    let heap_end = 0x100000000;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hello_rust() {
    printf("Hi!\n\0".as_ptr());
    println!("HI");
}

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

/*
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    unsafe { printf("Rust panic\0\n".as_ptr()) };
    loop {}
}*/
