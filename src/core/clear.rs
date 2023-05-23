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
}
