use std::mem::forget;

use super::*; 

impl <F, S> BiVec<F, S>  {
    pub fn into_vecs(self) -> (Vec<F>, Vec<S>) {
        let (first, second); 
        if self.capacity == 0 {
            first = Vec::new(); 
            second = Vec::new(); 
        } else {
            first = unsafe { Vec::from_raw_parts(self.first.as_ptr(), self.len, self.capacity) }; 
            second = unsafe { Vec::from_raw_parts(self.second.as_ptr(), self.len, self.capacity) };  
            forget(self); 
        }
        (first, second) 
    }
}

impl <F, S> Drop for BiVec<F, S> {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return; 
        } 
        self.clear(); 
        if core::mem::size_of::<F>() != 0 {
            let layout = core::alloc::Layout::array::<F>(self.capacity).unwrap(); 
            unsafe { std::alloc::dealloc(self.first.as_ptr() as *mut u8, layout); } 
        } 
        if core::mem::size_of::<S>() != 0 {
            let layout = core::alloc::Layout::array::<S>(self.capacity).unwrap(); 
            unsafe { std::alloc::dealloc(self.second.as_ptr() as *mut u8, layout); } 
        }
    }
}