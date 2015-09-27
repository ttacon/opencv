extern crate libc;

use libc::{c_int};

#[derive(Debug)]
#[repr(C)]
pub struct CvPoint {
    pub x: c_int,
    pub y: c_int,
}

#[link(name="opencv", kind="static")]
extern {
    fn cvPoint(x: c_int, y: c_int) -> CvPoint;
}

#[test]
fn it_works() {
    unsafe {
        let p = cvPoint(4, 5);
        println!("got: {:?}", p);
    }
}
