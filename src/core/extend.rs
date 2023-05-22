use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn extend_from_slice(&mut self, other: &[(F, S)]) {
        let len = self.len(); 
        let other_len = other.len(); 
        self.reserve(other_len); 
        unsafe {
            // let self_first_ptr = self.first.as_ptr().add(len); 
            // let self_second_ptr = self.second.as_ptr().add(len); 
            // let other_first_ptr = other.as_ptr() as *const F; 
            // let other_second_ptr = other.as_ptr() as *const S; 
            // ptr::copy_nonoverlapping(other_first_ptr, self_first_ptr, other_len); 
            // ptr::copy_nonoverlapping(other_second_ptr, self_second_ptr, other_len); 
        }
        self.len += other_len;  
    }
}