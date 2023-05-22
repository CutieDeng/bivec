use std::alloc::Layout;
use std::alloc::realloc;

use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn reserve(&mut self, additional: usize) {
        self.reserve_exact(additional); 
    } 
    pub fn reserve_exact(&mut self, additional: usize) {
        let addition_now = self.capacity - self.len; 
        if addition_now >= additional {
            return; 
        } 
        let Self {
            ref mut first, 
            ref mut second, 
            .. 
        } = *self; 
        let new_capacity = self.len + additional; 
        if std::mem::size_of::<F>() != 0 {
            let first_layout = Layout::array::<F>(new_capacity).unwrap(); 
            *first = NonNull::new(unsafe { realloc(first.as_ptr() as *mut u8, first_layout, new_capacity * std::mem::size_of::<F>()) as *mut F }).unwrap();
        }
        if std::mem::size_of::<S>() != 0 {
            let second_layout = Layout::array::<S>(new_capacity).unwrap(); 
            *second = NonNull::new(unsafe { realloc(second.as_ptr() as *mut u8, second_layout, new_capacity * std::mem::size_of::<S>()) as *mut S }).unwrap(); 
        }
    } 
}