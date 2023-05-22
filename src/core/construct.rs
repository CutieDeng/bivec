use super::*; 

impl <F, S> BiVec<F, S> {
    pub fn new() -> Self {
        Self {
            first: NonNull::dangling(),
            second: NonNull::dangling(),
            capacity: 0, 
            len: 0,
        } 
    } 
    pub fn with_capacity(capacity: usize) -> Self { 
        let (first, second); 
        if core::mem::size_of::<F>() == 0 {
            first = NonNull::dangling(); 
        } else {
            let layout = core::alloc::Layout::array::<F>(capacity).unwrap(); 
            let alloc = unsafe { std::alloc::alloc(layout) }; 
            first = NonNull::new(alloc as *mut F).unwrap(); 
        }
        if core::mem::size_of::<S>() == 0 {
            second = NonNull::dangling(); 
        } else {
            let layout = core::alloc::Layout::array::<S>(capacity).unwrap(); 
            let alloc = unsafe { std::alloc::alloc(layout) }; 
            second = NonNull::new(alloc as *mut S).unwrap();  
        }
        Self {
            first, 
            second, 
            capacity, 
            len: 0, 
        }
    }
}
