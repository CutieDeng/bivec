
use std::ptr::NonNull;

pub struct BiVec <F, S> {
    first: NonNull<F>, 
    second: NonNull<S>, 
    capacity: usize, 
    len: usize, 
}

mod destruct; 
mod construct;
mod reserve;
mod append;
mod property;
mod split;
mod dedup;
mod splice;
mod drain;
mod clear;
mod leak;
mod spare;
mod remove; 
mod extend;
mod raw; 
mod insert; 