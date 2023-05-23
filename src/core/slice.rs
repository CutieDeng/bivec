use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn as_slice(&self) -> (&[F], &[S]) {
        unsafe {
            let first = std::slice::from_raw_parts(self.first.as_ptr(), self.len); 
            let second = std::slice::from_raw_parts(self.second.as_ptr(), self.len); 
            (first, second)
        } 
    }
    pub fn as_slice_mut(&mut self) -> (&mut [F], &mut [S]) {
        unsafe {
            let first = std::slice::from_raw_parts_mut(self.first.as_ptr(), self.len); 
            let second = std::slice::from_raw_parts_mut(self.second.as_ptr(), self.len); 
            (first, second)
        } 
    } 
}