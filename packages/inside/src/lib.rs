extern "C" {
    fn host_hello(param: i32);
}

#[no_mangle]
extern "C" fn hello() {
    unsafe {
        host_hello(3);
    }
}
