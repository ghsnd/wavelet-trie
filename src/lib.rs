pub mod wavelet_trie;
mod bit_vec_wrap;

// note: moving pointers in rust:
// let x = Box::new(5);
// let ptr = Box::into_raw(x);
// let x = unsafe { Box::from_raw(ptr) };
// or can we move the box as such?
