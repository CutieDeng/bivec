use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn clear(&mut self) {
        for i in 0..self.len {
            unsafe { core::ptr::drop_in_place(self.first.as_ptr().add(i)); }  
        }
        for i in 0..self.len {
            unsafe { core::ptr::drop_in_place(self.second.as_ptr().add(i)); }  
        }
    }
    pub fn trim(&mut self) {
        let len = self.len; 
        let layout1 = core::alloc::Layout::array::<F>(len).unwrap(); 
        let layout2 = core::alloc::Layout::array::<S>(len).unwrap(); 
        let first = unsafe { std::alloc::alloc(layout1) as *mut F }; 
        let second = unsafe { std::alloc::alloc(layout2) as *mut S }; 
        unsafe {
            std::ptr::copy_nonoverlapping(self.first.as_ptr(), first, len); 
            std::ptr::copy_nonoverlapping(self.second.as_ptr(), second, len); 
        }  
        let layout1 = core::alloc::Layout::array::<F>(self.capacity).unwrap(); 
        let layout2 = core::alloc::Layout::array::<S>(self.capacity).unwrap(); 
        unsafe { std::alloc::dealloc(self.first.as_ptr() as *mut u8, layout1); } 
        unsafe { std::alloc::dealloc(self.second.as_ptr() as *mut u8, layout2); } 
        self.first = NonNull::new(first).unwrap(); 
        self.second = NonNull::new(second).unwrap(); 
        self.capacity = len;  
    }
}
