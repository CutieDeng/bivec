use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn leak(self) -> (&'static mut [F], &'static mut [S]) { 
        let first = unsafe { std::slice::from_raw_parts_mut(self.first.as_ptr(), self.len) }; 
        let second = unsafe { std::slice::from_raw_parts_mut(self.second.as_ptr(), self.len) }; 
        core::mem::forget(self);
        (first, second)
    } 
}