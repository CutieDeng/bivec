use std::ptr;

use super::*; 

impl <F, S> BiVec<F, S> {
    /// Appends the contents of `other` to `self`. 
    /// 
    /// # UNIMPLEMENTED 
    pub fn append(&mut self, other: &mut BiVec<F, S>) {
        self.reserve_exact(other.len); 
        unsafe {
            let first_ptr = self.first.as_ptr().add(self.len); 
            let second_ptr = self.second.as_ptr().add(self.len); 
            let other_first_ptr = other.first.as_ptr().add(other.len); 
            let other_second_ptr = other.second.as_ptr().add(other.len); 
            ptr::copy_nonoverlapping(other_first_ptr, first_ptr, other.len); 
            ptr::copy_nonoverlapping(other_second_ptr, second_ptr, other.len); 
        }
        self.len += other.len; 
        other.len = 0; 
    }
}