#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]

use std::net::TcpListener;

#[no_mangle]
pub extern "C" fn rust_start() {
    let builder = std::thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    /*let handler = builder
    .spawn(|| {
        // thread code
    })
    .unwrap();*/
    let a = vec![3, 4, 2, 1, 4, 2, 7];
    let b: Vec<_> = a.iter().map(|x| x + x).collect();
    for n in b {
        println!("hi from std {}", n);
    }

    let f = std::fs::write("hi.welt", "rust");

    let listener = TcpListener::bind("127.0.0.1:80");
    println!("created listener {:?}", listener);

    if let Ok(listener) = listener {
        for stream in listener.incoming() {
            println!("{:?}", stream.unwrap());
        }
    }
}
