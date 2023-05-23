use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn as_ptr(&self) -> (*const F, *const S) {
        (self.first.as_ptr(), self.second.as_ptr())
    } 
    pub fn as_mut_ptr(&mut self) -> (*mut F, *mut S) {
        (self.first.as_ptr(), self.second.as_ptr())
    } 
}