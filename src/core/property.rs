use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn len(&self) -> usize {
        self.len  
    }
    pub fn capacity(&self) -> usize {
        self.capacity 
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0 
    } 
}