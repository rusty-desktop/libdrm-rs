use std::ptr;
use libc::*;

#[repr(C)]
pub struct VolatileCell<T> {
     x: T
}

impl<T> VolatileCell<T> {
    unsafe fn get(&self) -> T {
        ptr::read_volatile(&self.x)
    }
    unsafe fn set(&mut self, x: T) {
       ptr::write_volatile(&mut self.x, x)
    }
}

pub type KernelSizeT = c_ulong;
