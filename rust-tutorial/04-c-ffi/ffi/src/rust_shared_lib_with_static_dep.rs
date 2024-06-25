use std::os::raw::c_int;

extern "C" {
    pub fn func() -> c_int;
}

pub fn f() {
    println!("hi {}",
             unsafe {
                 func()
             }
    );
}