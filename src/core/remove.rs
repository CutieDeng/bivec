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
    pub fn remove(&mut self, index: usize) -> (F, S) {
        let len = self.len(); 
        assert!(index < len, "remove index (is {}) should be < len (is {})", index, len); 
        let (f, s); 
        unsafe {
            let value_ptr = self.first.as_ptr().add(index); 
            let to_copy = self.first.as_ptr().add(index + 1); 
            f = ptr::read(value_ptr); 
            value_ptr.copy_from(to_copy, len - index - 1);
            let value_ptr = self.second.as_ptr().add(index); 
            let to_copy = self.second.as_ptr().add(index + 1); 
            s = ptr::read(value_ptr); 
            value_ptr.copy_from(to_copy, len - index - 1); 
        }
        (f, s)
    }
    pub fn retain(&mut self, mut f: impl FnMut(&F, &S) -> bool) {
        let len = self.len; 
        let mut retains = Vec::with_capacity(len);
        for i in 0..len {
            unsafe {
                let first_ptr = self.first.as_ptr().add(i); 
                let second_ptr = self.second.as_ptr().add(i); 
                if f(&*first_ptr, &*second_ptr) {
                    retains.push(i); 
                }
            }
        } 
        let mut reit = retains.iter(); 
        let mut current = reit.next(); 
        for i in 0..len {
            match current {
                Some(&current_val) => {
                    if i == current_val {
                        current = reit.next(); 
                        continue; 
                    } 
                }
                None => (), 
            }
            unsafe { self.first.as_ptr().add(i).drop_in_place(); } 
        }
        let mut reit = retains.iter(); 
        let mut current = reit.next(); 
        for i in 0..len {
            match current {
                Some(&current_val) => {
                    if i == current_val {
                        current = reit.next(); 
                        continue; 
                    } 
                }
                None => (), 
            }
            unsafe { self.second.as_ptr().add(i).drop_in_place(); } 
        }
        self.len = retains.len(); 
        for (i, &v) in retains.iter().enumerate() {
            unsafe {
                let first_ptr = self.first.as_ptr().add(i); 
                let first_to_copy = self.first.as_ptr().add(v); 
                first_ptr.write(first_to_copy.read()); 
            } 
        }
        for (i, &v) in retains.iter().enumerate() {
            unsafe {
                let second_ptr = self.second.as_ptr().add(i); 
                let second_to_copy = self.second.as_ptr().add(v); 
                second_ptr.write(second_to_copy.read()); 
            } 
        } 
    }
}