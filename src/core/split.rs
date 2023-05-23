use super::*; 

impl <F, S> BiVec<F, S> {
    /// # Unimplement 
    pub fn split_at(&mut self, at: usize) -> ! {
        assert!(at <= self.len, "index out of bounds"); 
        unimplemented!()
    }
    /// # Unimplement 
    pub fn split_off(&mut self, at: usize) -> ! {
        assert!(at <= self.len, "index out of bounds"); 
        unimplemented!()
    }
}