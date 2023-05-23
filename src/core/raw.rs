use super::*; 

impl <F, S> BiVec<F, S> {
    pub unsafe fn from_raw_parts(ptr1: *mut F, ptr2: *mut S, length: usize, capacity: usize) -> BiVec<F, S>{
        BiVec {
            first: NonNull::new_unchecked(ptr1), 
            second: NonNull::new_unchecked(ptr2), 
            len: length, 
            capacity: capacity, 
        } 
    }
}