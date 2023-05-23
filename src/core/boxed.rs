use std::mem::forget;

use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn into_boxed_slice(mut self) -> (Box<[F]>, Box<[S]>) {
        self.trim(); 
        let (first, second) = self.as_slice_mut(); 
        let first = unsafe { Box::from_raw(first) }; 
        let second = unsafe { Box::from_raw(second) }; 
        forget(self);  
        (first, second)
    }
}