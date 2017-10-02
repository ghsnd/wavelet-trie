use bit_vec_wrap::BitVecWrap;
use std::vec::Vec;

// see paper:
// R. Grossi, G. Ottoviano "The Wavelet Trie: Maintaining an Indexed Sequence of Strings in Compressed Space"
// strings are assumed prefix-free. This can be solved by appending a terminator symbol at the end of the string.

// a node in the wavelet trie
#[derive(Debug)]
pub struct WaveletTrie {
	prefix: BitVecWrap,               // α in the literature
	positions: BitVecWrap,            // β in the literature
	left: Option<Box<WaveletTrie>>,   // left subtrie, if any
	right: Option<Box<WaveletTrie>>   // right subtrie, if any

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

	// TODO: turn this into static constructor
	pub fn insert_static(&mut self, sequences: &[BitVecWrap]) {
		if !sequences.is_empty() {
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
				self.prefix = first_sequence.clone();
				for sequence in sequences {
					self.prefix = self.prefix.longest_common_prefix(sequence);
				}
				// split accordingly
				let mut left_sequences: Vec<BitVecWrap> = Vec::new();
				let mut right_sequences: Vec<BitVecWrap> = Vec::new();
				for sequence in sequences {
					let (bit, suffix) = sequence.different_suffix(self.prefix.len());
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

	pub fn insert(&mut self, sequence: &BitVecWrap, index: usize) {
		// if sequence == prefix {
		//   if no children => OK, return
		//   else => prefix is prefix of sequence; not allowed!
		// } else if sequence is prefix of prefix {
		//   sequence is prefix of prefix; not allowed!
		// } else {
		//   lcp = longest common prefix
		//   if lcp == prefix
		//   if lcp empty && prefix not empty =>
		// }

		if *sequence == self.prefix {
			if self.left.is_some() {
				// prefix is prefix of sequence; not allowed!
				panic!("strings are not prefix-free anymore. Cannot do this.");
			} else {
				// OK
				return
			}
		} else if sequence.is_prefix_of(&self.prefix) {
			panic!("strings are not prefix-free anymore. Cannot do this.");
		} else {
			if self.prefix.is_empty() {
				//TODO
			} else {
				// TODO
			}
			let lcp = sequence.longest_common_prefix(&self.prefix);
			// bit_1 determines wheter original node comes as left or right child in of new node
			// suffix_self becomes prefix in new split node
			let (bit_1, suffix_self) = self.prefix.different_suffix(lcp.len());
			// suffix_seq becomes prefix in new leaf
			let (bit_2, suffix_seq) = sequence.different_suffix(lcp.len());

			if lcp == self.prefix {
				// we need to recurse further
				if bit_2 {
					if let Some(ref mut child) = self.right {
						child.insert(&suffix_seq, 0);
					}
				} else {
					if let Some(ref mut child) = self.left {
						child.insert(&suffix_seq, 0);
					}
				}
			} else {

				// reconstruct the original node
				let original_left = self.left.take();
				let original_right = self.right.take();
				let original_positions = self.positions.copy();
				let original_node = WaveletTrie {
					left: original_left,
					right: original_right,
					prefix: lcp,
					positions: original_positions
				};
	
				// create the leaf
				let new_leaf = WaveletTrie {
					left: None,
					right: None,
					prefix: suffix_seq,
					//positions : BitVecWrap::from_elem(original_positions.len() /* check this! */, bit_2)
					positions : BitVecWrap::new()
				};

				// make this node the new node
				let (new_left, new_right) = match bit_1 {
					false => (Some(Box::new(original_node)), Some(Box::new(new_leaf))),
					true => (Some(Box::new(new_leaf)), Some(Box::new(original_node)))
				};
				self.left = new_left;
				self.right = new_right;
				self.prefix = suffix_self;
			}
		}
	}

	// count the number of occurrences "sequence" (can be a prefix) up to index − 1.
	// returns None if sequence does not occur
	pub fn rank(&self, sequence: &BitVecWrap, index: usize) -> Option<usize> {
		if sequence.is_empty() || sequence == &self.prefix {
			Some(index)
		} else if sequence.len() < self.prefix.len() {
			// sequence has to be a prefix of "prefix"
			// if so, return "index". If not, the sequence is not in the trie.
			match sequence.is_prefix_of(&self.prefix) {
				true => Some(index),
				false => None
			}
		} else {
			// "prefix" has to be a prefix of sequence
			// if so, substract "prefix" from the beginning of sequence, and recurse!
			match self.prefix.is_prefix_of(sequence) {
				true => {
					let (bit, suffix) = sequence.different_suffix(self.prefix.len());
					match bit {
						// TODO: rewrite with closures?
						true => {
							let new_index = self.positions.rank_one(index);
							match self.right {
								Some(ref trie) => trie.rank(&suffix, new_index),
								None => Some(new_index)
							}
						},
						false => {
							let new_index = self.positions.rank_zero(index);
							match self.left {
								Some(ref trie) => trie.rank(&suffix, new_index),
								None => Some(new_index)
							}
						}
					}
				},
				false => None
			}
		}
	}
}

mod tests;
