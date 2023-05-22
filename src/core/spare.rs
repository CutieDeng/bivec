use std::mem::MaybeUninit;

use super::*; 

impl <F, S> BiVec<F, S> {
    /// Returns the remaining spare capacity of the vector as two slices of `MaybeUninit<F>` and `MaybeUninit<S>`. 
    /// 
    /// The returned slices can be used to fill the BiVec with data (e.g. by reading from a file) before marking the data as initialized using the `set_len` method. 
    pub fn spare_capacity_mut(&mut self) -> (&mut [MaybeUninit<F>], &mut [MaybeUninit<S>]) {
        let first = unsafe { std::slice::from_raw_parts_mut(self.first.as_ptr().add(self.len).cast(), self.capacity - self.len) }; 
        let second = unsafe { std::slice::from_raw_parts_mut(self.second.as_ptr().add(self.len).cast(), self.capacity - self.len) }; 
        (first, second) 
    }
    /// Forces the length of the BiVec to `new_len`. 
    /// 
    /// This is a low-level operation that maintains none of the normal invariants of the type. 
    /// Normally changing the length of a vector is done using other safe operations instead. 
    /// 
    /// # Safety 
    /// 
    /// - `new_len` must be less than or equal to `capacity()`. 
    /// - The elements at old_len..new_len must be initialized. 
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity); 
        self.len = new_len;  
    }
}