use std::ops::RangeBounds;

use super::*; 

impl <F, S> BiVec<F, S> {
    /// # Unimplemented 
    pub fn splice(&mut self, range: impl RangeBounds<usize>, replace_with: impl IntoIterator<Item = (F, S)>) 
        -> ! 
        // -> impl Iterator<Item = (F, S)>  
    {
        unimplemented!()
    }
}