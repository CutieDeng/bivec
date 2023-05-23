use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn push(&mut self, f: F, s: S) {
        let len = self.len(); 
        self.reserve(1); 
        unsafe {
            let first_ptr = self.first.as_ptr().add(len); 
            let second_ptr = self.second.as_ptr().add(len); 
            first_ptr.write(f); 
            second_ptr.write(s); 
        }
        self.len += 1;  
    }
    pub fn insert(&mut self, index: usize, f: F, s: S) {
        let len = self.len(); 
        assert!(index <= len, "index out of bounds"); 
        self.reserve(1); 
        unsafe {
            let first_ptr = self.first.as_ptr().add(index); 
            let second_ptr = self.second.as_ptr().add(index); 
            if index != len {
                std::ptr::copy(first_ptr, first_ptr.add(1), len - index); 
                std::ptr::copy(second_ptr, second_ptr.add(1), len - index); 
            }
            first_ptr.write(f); 
            second_ptr.write(s); 
        } 
        self.len += 1; 
    }
}