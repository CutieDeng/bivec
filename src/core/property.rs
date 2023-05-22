use super::*; 

impl <F, S> BiVec<F, S> {
    #[inline]
    pub fn len(&self) -> usize {
        self.len  
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity 
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0 
    } 
}