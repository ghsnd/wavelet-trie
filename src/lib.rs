use bit_vec_wrap::BitVecWrap;

// a node in the wavelet trie
pub struct WaveletTrie {
	left: Option<Box<WaveletTrie>>,  // left subtrie, if any
	right: Option<Box<WaveletTrie>>, // right subtrie, if any
	prefix: BitVecWrap,                  // α in the literature
	positions: BitVecWrap                // β in the literature
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
}

mod bit_vec_wrap;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
	}
}
