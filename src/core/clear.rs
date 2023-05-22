use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn clear(&mut self) {
        if core::mem::size_of::<F>() == 0 { 
            for _ in 0..self.len {
                unsafe { core::ptr::drop_in_place(&mut core::mem::zeroed::<F>()); } 
            } 
        } else {
            for i in 0..self.len {
                unsafe { core::ptr::drop_in_place(self.first.as_ptr().add(i)); }  
            }
        }
        if core::mem::size_of::<S>() == 0 { 
            for _ in 0..self.len {
                unsafe { core::ptr::drop_in_place(&mut core::mem::zeroed::<S>()); } 
            } 
        } else {
            for i in 0..self.len {
                unsafe { core::ptr::drop_in_place(self.second.as_ptr().add(i)); }  
            }
        } 
    }
}
