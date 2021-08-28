#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]

use std::net::TcpListener;

#[no_mangle]
pub extern "C" fn rust_start() {
    let a = vec![3, 4, 2, 1, 4, 2, 7];
    let b: Vec<_> = a.iter().map(|x| x + x).collect();
    for n in b {
        println!("hi from std {}", n);
    }
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    println!("created listener ");

    for stream in listener.incoming() {
        println!("{:?}", stream.unwrap());
    }
}
