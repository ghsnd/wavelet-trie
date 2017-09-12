use bit_vec_wrap::BitVecWrap;
use std::vec::Vec;

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

	pub fn insert_static(&mut self, sequences: &[BitVecWrap]) {
		if (!sequences.is_empty()) {
			// first check if all bitvectors in the sequence are the same
			let first_sequence = &sequences[0];
			let all_equal = sequences.iter().all( |current_sequence| current_sequence == first_sequence);
			if all_equal {
				self.prefix = first_sequence.clone();
			} else {
				// create children
				let mut left_child = WaveletTrie::new();
				let mut right_child = WaveletTrie::new();
				// find longest common prefix
				self.prefix = 
					sequences.iter()
							.fold(BitVecWrap::new(),
								|prefix, current_sequence| prefix.longest_common_prefix(current_sequence)
							);
				// split accordingly
				let mut left_sequences: Vec<BitVecWrap> = Vec::new();
				let mut right_sequences: Vec<BitVecWrap> = Vec::new();
				for sequence in sequences {
					let (bit, suffix) = sequence.different_suffix(&self.prefix);
					self.positions.push(bit);
					if bit {
						right_sequences.push(suffix);
					} else {
						left_sequences.push(suffix);
					}
				}
				// now insert left and right sequences into subtrees
				left_child.insert_static(&left_sequences);
				right_child.insert_static(&right_sequences);
				self.left = Some(Box::new(left_child));
				self.right = Some(Box::new(right_child));
			}
		}
	}



	// this has to return something to indicate modifications needed in the parent
/*	pub fn insert(&mut self, index: u64, value: BitVecWrap /* or should this be a &str? */) {

		if !self.left.is_some() && !self.right.is_some() && self.prefix.is_empty() {
			// if this node is completely empty, just insert the value. The index is automatically equal to 0
			self.prefix = value.clone();
		} else {
			let common_prefix = self.prefix.longest_common_prefix(&value);
		}
	}*/
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
