use bit_vec_wrap::BitVecWrap;

// see paper:
// R. Grossi, G. Ottoviano "The Wavelet Trie: Maintaining an Indexed Sequence of Strings in Compressed Space"
// strings are assumed prefix-free. This can be solved by appending a terminator symbol at the end of the string.

// a node in the wavelet trie
pub struct WaveletTrie {
	left: Option<Box<WaveletTrie>>,   // left subtrie, if any
	right: Option<Box<WaveletTrie>>,  // right subtrie, if any
	prefix: BitVecWrap,               // α in the literature
	positions: BitVecWrap             // β in the literature
}


impl WaveletTrie {

	// constructor
	pub fn new() -> Self {
		WaveletTrie {
			left: None,
			right: None,
			prefix: BitVecWrap::new(),
			positions: BitVecWrap::new()
		}
	}

	// this has to return something to indicate modifications needed in the parent
	pub fn insert(&mut self, index: u64, value: BitVecWrap /* or should this be a &str? */) {

		if !self.left.is_some() && !self.right.is_some() && self.prefix.is_empty() {
			// if this node is completely empty, just insert the value. The index is automatically equal to 0
			self.prefix = value.clone();
		} else {
			let common_prefix = self.prefix.longest_common_prefix(&value);
		}
	}
}

mod bit_vec_wrap;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
	}
}

// note: moving pointers in rust:
// let x = Box::new(5);
// let ptr = Box::into_raw(x);
// let x = unsafe { Box::from_raw(ptr) };
// or can we move the box as such?
