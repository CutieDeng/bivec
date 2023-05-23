use std::ops::RangeBounds;

use super::*; 

impl <F: Clone, S: Clone> BiVec<F, S> {
    pub fn resize(&mut self, new_len: usize, value: (F, S)) {
        self.resize_with(new_len, move || value.clone())
    } 
    pub fn resize_with(&mut self, new_len: usize, mut f: impl FnMut() -> (F, S)) {
        if new_len >= self.len {
            self.reserve(new_len - self.len);
            for _ in self.len..new_len {
                let (f, s) = f(); 
                self.push(f, s);
            }
        } else {
            self.truncate(new_len); 
        } 
    } 
    pub fn extend_from_slice(&mut self, other: &[(F, S)]) {
        let len = self.len(); 
        let other_len = other.len(); 
        self.reserve(other_len); 
        unsafe {
            let first_ptr = self.first.as_ptr().add(len); 
            let second_ptr = self.second.as_ptr().add(len); 
            for i in 0..other.len() {
                let first_ptr = first_ptr.add(i); 
                let second_ptr = second_ptr.add(i); 
                first_ptr.write( other[i].0.clone() );
                second_ptr.write( other[i].1.clone() ); 
            }
        }
        self.len += other_len;  
    }
    pub fn extend_from_within(&mut self, src: impl RangeBounds<usize>) {
        let len = self.len(); 
        let start = src.start_bound();
        let end = src.end_bound(); 
        let start = match start {
            std::ops::Bound::Included(value) => *value, 
            std::ops::Bound::Excluded(value) => *value + 1,
            std::ops::Bound::Unbounded => 0, 
        }; 
        let end = match end {
            std::ops::Bound::Included(value) => *value + 1, 
            std::ops::Bound::Excluded(value) => *value,
            std::ops::Bound::Unbounded => self.len(), 
        }; 
        let src_len = end - start; 
        self.reserve(src_len); 
        unsafe {
            let first_ptr = self.first.as_ptr().add(len); 
            let second_ptr = self.second.as_ptr().add(len); 
            let src_first_ptr = self.first.as_ptr().add(start); 
            let src_second_ptr = self.second.as_ptr().add(start); 
            for i in 0..src_len {
                let first_ptr = first_ptr.add(i); 
                let second_ptr = second_ptr.add(i); 
                let src_first_ptr = src_first_ptr.add(i); 
                let src_second_ptr = src_second_ptr.add(i); 
                let src_first_ptr = src_first_ptr.as_ref().unwrap(); 
                let src_second_ptr = src_second_ptr.as_ref().unwrap(); 
                first_ptr.write( src_first_ptr.clone() ); 
                second_ptr.write( src_second_ptr.clone() ); 
            } 
        }
        self.len += src_len;  
    }
}