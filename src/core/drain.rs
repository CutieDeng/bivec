use std::ops::RangeBounds;

use super::*; 

impl <F, S> BiVec<F, S> {
    /// # Unimplemented 
    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> ! {
        unimplemented!()
    } 
}