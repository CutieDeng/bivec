use std::ptr;

use super::*; 

impl <F, S> BiVec<F, S> {
    /// Removes the element at the given index, and returns it. 
    /// 
    /// The removed element is replaced by the last element of the vector. 
    pub fn swap_remove(&mut self, first_index: usize, second_index: usize) -> (F, S) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("swap_remove index (is {index}) should be < len (is {len})");
        }
        let len = self.len();
        if first_index >= len {
            assert_failed(first_index, len);
        }
        if second_index >= len {
            assert_failed(second_index, len); 
        }
        self.len -= 1; 
        let (first, second); 
        unsafe {
            let value_ptr = self.first.as_ptr().add(first_index); 
            let end_ptr = self.first.as_ptr().add(len); 
            first = ptr::read(value_ptr); 
            value_ptr.write(end_ptr.read());
            let value_ptr = self.second.as_ptr().add(second_index); 
            let end_ptr = self.second.as_ptr().add(len); 
            second = ptr::read(value_ptr); 
            value_ptr.write(end_ptr.read()); 
        }
        (first, second)
    }
    pub fn swap_remove_same(&mut self, index: usize) -> (F, S) {
        self.swap_remove(index, index)
    } 
}