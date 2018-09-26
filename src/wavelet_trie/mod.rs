extern crate dyn_bit_vec;
extern crate serde;
extern crate bincode;

use self::dyn_bit_vec::DBVec;
use std::fmt;
use std::vec::Vec;
use std::string::FromUtf8Error;
use std::io::{Read, Write};
use self::bincode::{serialize_into, deserialize_from};

// based on the paper:
// R. Grossi, G. Ottoviano "The Wavelet Trie: Maintaining an Indexed Sequence of Strings in Compressed Space"
// strings are assumed prefix-free. This can be solved by appending a terminator symbol at the end of the string.

// a node in the wavelet trie
#[derive(Clone, Serialize, Deserialize)]
pub struct WaveletTrie {
	prefix: DBVec,                  // α in the literature
	positions: DBVec,               // β in the literature
	left: Option<Box<WaveletTrie>>,   // left subtrie, if any
	right: Option<Box<WaveletTrie>>   // right subtrie, if any

}

impl WaveletTrie {

	// constructor
	pub fn new() -> Self {
		WaveletTrie {
			left: None,
			right: None,
			prefix: DBVec::new(),
			positions: DBVec::new()
		}
	}

	pub fn from_sequences(sequences: &[DBVec]) -> Self {
		let mut wavelet_trie = WaveletTrie::new();
		wavelet_trie.insert_static(sequences);
		wavelet_trie
	}

	fn insert_static(&mut self, sequences: &[DBVec]) {
		if !sequences.is_empty() {
			let first_sequence = &sequences[0];
			let all_equal = sequences.iter().all( |current_sequence| current_sequence == first_sequence);
			if all_equal {
				self.prefix = first_sequence.clone();
				self.positions = DBVec::from_elem(sequences.len() as u64, false);
			} else {
				// create children
				let mut left_child = WaveletTrie::new();
				let mut right_child = WaveletTrie::new();
				// find longest common prefix
				self.prefix = first_sequence.clone();
				for sequence in sequences {
					let new_prefix = self.prefix.longest_common_prefix(sequence);
					if new_prefix.len() < self.prefix.len() {
						self.prefix = new_prefix;
					}
				}
				// split accordingly
				let mut left_sequences: Vec<DBVec> = Vec::new();
				let mut right_sequences: Vec<DBVec> = Vec::new();
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

	pub fn generate_graph(&self, out: &mut Write) {
		self.generate_graph_internal(0, out);
	}

	fn generate_graph_internal(&self, node_nr: u64, out: &mut Write) {
		if node_nr == 0 {
			out.write(b"digraph wavelet_trie {\n  graph [rankdir = \"LR\"]; node [shape = \"record\"];\n").unwrap();
		}

		out.write_fmt(format_args!(" \"{}\" [label = \"{}|pref len: {}|pref spars: {}|pos len:{}|pos spars: {}\"];\n",
			node_nr,
			node_nr,
			self.prefix.len(),
			self.prefix.sparseness(),
			self.positions.len(),
			self.positions.sparseness()
		)).unwrap();

		if let Some(ref child) = self.left {
			let child_nr = node_nr * 2 + 1;
			child.generate_graph_internal(child_nr, out);
			out.write_fmt(format_args!(" \"{}\" -> \"{}\"\n", node_nr, child_nr)).unwrap();
		}

		if let Some(ref child) = self.right {
			let child_nr = node_nr * 2 + 2;
			child.generate_graph_internal(child_nr, out);
			out.write_fmt(format_args!(" \"{}\" -> \"{}\"\n", node_nr, child_nr)).unwrap();
		}

		if node_nr == 0 {
			out.write(b"}\n").unwrap();
		}
	}

	pub fn print_stats(&self) -> (usize, usize, usize) { // nr subnodes, used bits, allocated bits
		let mut nr_subnodes = 0;
		let mut used_bits = (self.prefix.len() + self.positions.len()) as usize;
		let mut allocated_bits = self.prefix.allocated_bytes() * 8 + self.positions.allocated_bytes() * 8 + 64;
		//println!("prefix len: {}\tpositions len: {}\tallocated bits: {}\tused bits:{}", self.prefix.len(), self.positions.len(), allocated_bits, used_bits);
		//println!("prefix: {:?}", self.prefix);
		//println!("positions: {:?}", self.positions);
		if let Some(ref child) = self.left {
			let (nn, ub, ab) = child.print_stats();
			used_bits += ub;
			allocated_bits += ab;
			nr_subnodes += 1 + nn;
		}
		if let Some(ref child) = self.right {
			let (nn, ub, ab) = child.print_stats();
			used_bits += ub;
			allocated_bits += ab;
			nr_subnodes += 1 + nn;
		}
		(nr_subnodes, used_bits, allocated_bits)
	}

	// append a sequence to the trie at last position
	pub fn append(&mut self, sequence: &DBVec) -> Result<(), &'static str> {
		let index = self.positions.len();
		self.insert(sequence, index)
	}

	pub fn insert(&mut self, sequence: &DBVec, index: u64) -> Result<(), &'static str> {
		// 1. self.prefix is empty, no children:
		//     self.prefix = sequence
		// 2. self.prefix is empty, children:
		//     2.a. sequence is empty:
		//         ERROR: sequence a prefix of self.prefix
		//     2.b. sequence not empty:
		//         Take the first bit off of sequence; this determines whether the rest of sequence is inserted into the left or right child
		// 3. self.prefix not empty:
		//     3.a. sequence is empty:
		//         ERROR: sequence is prefix of string already in the trie
		//     3.b. self.prefix == sequence:
		//         if no children: OK
		//         if children: ERROR: sequence is prefix of string already in trie
		//     3.c. sequence is prefix of self.prefix:
		//         ERROR
		//     3.d. self.prefix is prefix of sequence:
		//         if children: substract self.prefix from sequence, take the first bit off of sequence; this determines whether the rest of sequence is inserted into the left or right child
		//         if no children: ERROR
		//     else:
		//         (split current node; one child is existing trie and the other a new leaf)
		//         calculate longest common prefix (lcp) of self.prefix and sequence
		//         one new node has as prefix the suffix of self.prefix and the original children
		//         one new node had as prefix the suffix of sequence and no children
		//         self.prefix = lcp; self.left and self.right are the new nodes, determined by the first buit of the calculated suffixes

		if self.prefix.is_empty() {
			// case 1: empty prefix, no children
			if self.left.is_none() {
				self.prefix = sequence.copy();
				self.positions.push(false);
				return Ok(());

			// case 2: empty prefix, children
			} else {
				if sequence.is_empty() {
					return Err("The string being inserted is a prefix of a string in the trie, which is not allowed. (1)");
				} else {
					return self.insert_to_child(sequence, index);
				}
			}
		}

		// case 3: prefix is not empty
		else {
			if sequence.is_empty() {
				return Err("The string being inserted is a prefix of a string in the trie, which is not allowed. (5)");
			} else if &self.prefix == sequence {
				if self.left.is_none() {
					self.positions.insert(false, index);
					return Ok(());
				} else {
					return Err("The string being inserted is a prefix of a string in the trie, which is not allowed. (2)");
				}
			} else if self.prefix.starts_with(&sequence) {
				return Err("The string being inserted is a prefix of a string in the trie, which is not allowed. (3)");
			} else if sequence.starts_with(&self.prefix) {
				if self.left.is_none() {
					//println!("prefix:\n>>{:?}, sequence:\n{:?}", self.prefix, sequence);
					return Err("The string being inserted is a prefix of a string in the trie , which is not allowed. (4)");
				} else {
					return self.insert_to_child(sequence, index);
				}
			} else {
				let lcp = sequence.longest_common_prefix(&self.prefix);
				// bit_self determines wheter original node comes as left or right child in of new node
				// suffix_self becomes prefix in new split node
				let (bit_self, suffix_self) = self.prefix.different_suffix(lcp.len());
				// suffix_seq becomes prefix in new leaf
				let (bit_seq, suffix_seq) = sequence.different_suffix(lcp.len());

				// reconstruct the original node
				let original_left = self.left.take();
				let original_right = self.right.take();
				let original_positions = self.positions.copy();
				let original_node = WaveletTrie {
					left: original_left,
					right: original_right,
					prefix: suffix_self,
					positions: original_positions
				};

				// create the leaf
				let new_leaf = WaveletTrie {
					left: None,
					right: None,
					prefix: suffix_seq,
					positions: DBVec::from_elem(1, false)
				};

				// make this node the new node
				let (new_left, new_right) = match bit_self {
					false => (Some(Box::new(original_node)), Some(Box::new(new_leaf))),
					true => (Some(Box::new(new_leaf)), Some(Box::new(original_node)))
				};
				self.left = new_left;
				self.right = new_right;
				self.prefix = lcp;
				let pos_len = self.positions.len();
				self.positions = DBVec::from_elem(pos_len, bit_self);
				self.positions.insert(bit_seq, index);

				return Ok(());
			}
		}
	}

	fn insert_to_child(&mut self, sequence: &DBVec, index: u64) -> Result<(), &'static str> {
		let (bit, suffix) = sequence.different_suffix(self.prefix.len());
		self.positions.insert(bit, index);
		let new_pos = self.positions.rank(bit, index);
		match bit {
			true => {
				if let Some(ref mut child) = self.right {
					child.insert(&suffix, new_pos)
				} else {
					Err("The right child has run away!")
				}
			},
			false => {
				if let Some(ref mut child) = self.left {
					child.insert(&suffix, new_pos)
				} else {
					Err("The left child has run away!")
				}
			}
		}
	}

	// counts the number of occurrences "sequence" (can be a prefix) up to index − 1.
	// returns None if sequence does not occur
	pub fn rank(&self, sequence: &DBVec, index: u64) -> Option<u64> {
		if self.prefix.is_empty() && self.positions.is_empty() {
			None
		} else if sequence.is_empty() || sequence == &self.prefix {
			Some(index)
		} else if sequence.len() < self.prefix.len() {
			// sequence has to be a prefix of "prefix"
			// if so, return "index". If not, the sequence is not in the trie.
			match self.prefix.starts_with(sequence) {
				true => Some(index),
				false => None
			}
		} else {
			// "prefix" has to be a prefix of sequence
			// if so, substract "prefix" from the beginning of sequence, and recurse!
			match sequence.starts_with(&self.prefix) {
				true => {
					let (bit, suffix) = sequence.different_suffix(self.prefix.len());
					let new_index = self.positions.rank(bit, index);
					match bit {
						true => {
							match self.right {
								Some(ref trie) => trie.rank(&suffix, new_index),
								None => Some(new_index)
							}
						},
						false => {
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

	pub fn len(&self) -> u64 {
		self.positions.len()
	}

	// retrieve the sequence at the given index
	pub fn access(&self, index: u64) -> DBVec {
		let mut result = self.prefix.copy();
		if self.left.is_some() {	// if NO children, the position vector doesn't count...
			let bit = self.positions.get(index);
			let new_index = self.positions.rank(bit, index);
			result.push(bit);
			match bit {
				true => {
					if let Some(ref trie) = self.right {
						result.append_vec(&mut trie.access(new_index));
					}
				},
				false => {
					if let Some(ref trie) = self.left {
						result.append_vec(&mut trie.access(new_index));
					}
				}
			};
		}
		result
	}

	// find the position of the occurrence_nr-th given sequence (can be a prefix)
	// an occurrence number starts at 1 (a zero-th occurrence makes no sense)
	// returns None if not found.
	pub fn select(&self, sequence: &DBVec, occurrence_nr: u64) -> Option<u64> {
		// find recursively until node where sequence matches or is prefix of self.prefix.
		// upon return, calculate back the positions of [bit], depending on the value of bit.
		if sequence.is_empty() || sequence == &self.prefix || self.prefix.starts_with(sequence) {
			// OK, found!
			Some(occurrence_nr - 1)	// -1 due to +1 offset of occurrence_nr
		} else if sequence.starts_with(&self.prefix) {
			if self.left.is_none() {
				// domage, sequence not in trie!
				None
			} else {
				// search further
				let (bit, suffix) = sequence.different_suffix(self.prefix.len());

				// closure that is used to calculate the new position.
				let calc_new_pos = |trie: &WaveletTrie| {
					let pos_option = trie.select(&suffix, occurrence_nr);
					match pos_option {
						Some(pos) => self.positions.select(bit, pos + 1),
						None => None
					}
				};
				match bit {
					true => {
						if let Some(ref trie) = self.right { // is always true in this case
							return calc_new_pos(trie);
						}
					},
					false => {
						if let Some(ref trie) = self.left { // is always true in this case
							return calc_new_pos(trie);
						}
					}
				}
				// should never come here. Maybe panic?
				None
			}
		} else {
			// domage, sequence not in trie!
			None
		}
	}

	// find the positions of all occurrences of the given sequence (can be prefix)
	pub fn select_all(&self, sequence: &DBVec) -> Vec<u64> {
		if sequence.is_empty() || sequence == &self.prefix || self.prefix.starts_with(sequence) {
			// found! return vector [0, 1, ... positions.len() - 1]
			let mut result = Vec::with_capacity(self.positions.len() as usize);
			for i in 0..self.positions.len() {
				result.push(i);
			}
			result
		} else if sequence.starts_with(&self.prefix) {
			if self.left.is_none() {
				// domage, sequence not in trie!
				Vec::new()
			} else {
				// search further
				let (bit, suffix) = sequence.different_suffix(self.prefix.len());

				// closure that is used to calculate the new position.
				let calc_new_pos = |trie: &WaveletTrie| {
					let mut all_positions = trie.select_all(&suffix);	// positions is now a Vec
					for position in all_positions.iter_mut() {
						let new_position = self.positions.select(bit, *position + 1);
						match new_position {
							Some(new_pos) => {*position = new_pos;},
							None => {panic!("This cannot happen!");},
						};
					}
					all_positions
				};

				match bit {
					true => {
						if let Some(ref trie) = self.right { // is always true in this case
							return calc_new_pos(trie);
						}
					},
					false => {
						if let Some(ref trie) = self.left { // is always true in this case
							return calc_new_pos(trie);
						}
					}
				}
				// should never come here. Maybe panic?
				Vec::new()
			}
		} else {
			// domage, sequence not in trie!
			Vec::new()
		}
	}

	pub fn delete(&mut self, index: u64) {
		let bit = self.positions.get(index);
		let new_pos = self.positions.rank(bit, index);
		match bit {
			true => {
				let mut delete_child = false;
				if let Some(ref mut child) = self.right {
					child.delete(new_pos);
					if child.len() == 0 {	// this would be the leaf node
						delete_child = true;
					}
				}
				if delete_child {
					self.right = None;
					self.prefix.push(!bit);
					// merge other child with this node
					let mut new_left: Option<Box<WaveletTrie>> = None;
					let mut new_right: Option<Box<WaveletTrie>> = None;
					if let Some(ref mut child) = self.left {
						self.prefix.append_vec(&mut child.prefix);
						new_left = child.left.take();
						new_right = child.right.take();
					}
					self.left = new_left;
					self.right = new_right;
				}
			},
			false => {
				let mut delete_child = false;
				if let Some(ref mut child) = self.left {
					child.delete(new_pos);
					if child.len() == 0 {	// this would be the leaf node
						delete_child = true;
					}
				}
				if delete_child {
					self.left = None;
					self.prefix.push(!bit);
					// merge other child with this node
					let mut new_left: Option<Box<WaveletTrie>> = None;
					let mut new_right: Option<Box<WaveletTrie>> = None;
					if let Some(ref mut child) = self.right {
						self.prefix.append_vec(&mut child.prefix);
						new_left = child.left.take();
						new_right = child.right.take();
					}
					self.left = new_left;
					self.right = new_right;
				}
			}
		}
		self.positions.delete(index);
		if self.len() == 0 {
			// the trie is in fact empty!
			self.prefix = DBVec::new();
		}
		if self.left.is_none() { // if no children, set the positions all to zero
			self.positions.set_none();
		}
	}

	// appends a string to the trie
	pub fn append_str(&mut self, text: &str) -> Result<(), &'static str> {
		self.append(&Self::text_to_bitvec(text))
	}

	// counts the number of occurrences "text" (can be a prefix) up to index - 1.
	// returns None if the string does not occur
	pub fn rank_str(&self, text: &str, index: u64) -> Option<u64> {
		let sequence = DBVec::from_bytes(text.as_bytes());
		self.rank(&sequence, index)
	}

	// retrieves the string at the given index
	pub fn access_str(&self, index: u64) -> Result<String, FromUtf8Error> {
		let sequence = self.access(index);
		Self::bitvec_to_text(&sequence)
	}

	// finds the position of the occurrence_nr-th given string (can be a prefix)
	// an occurrence number starts at 1 (a zero-th occurrence makes no sense)
	// returns None if not found.
	pub fn select_str(&self, text: &str, occurrence_nr: u64) -> Option<u64> {
		let sequence = DBVec::from_bytes(text.as_bytes());
		self.select(&sequence, occurrence_nr)
	}

	fn text_to_bitvec(text: &str) -> DBVec {
		let text_bytes = text.as_bytes();
		let mut text_bitvec = DBVec::from_bytes(text_bytes);
		// add the terminator!
		let mut end_symbol = DBVec::from_bytes(&[0b0]);
		text_bitvec.append_vec(&mut end_symbol);
		text_bitvec
	}

	// finds the positions of all occurrences of the given sequence (can be prefix)
	pub fn select_all_str(&self, text: &str) -> Vec<u64> {
		let sequence = DBVec::from_bytes(text.as_bytes());
		self.select_all(&sequence)
	}

	fn bitvec_to_text(sequence: &DBVec) -> Result<String, FromUtf8Error> {
		let mut bytes = sequence.to_bytes();
		// destroy the terminator!
		bytes.pop();
		String::from_utf8(bytes)
	}

	fn fmt_pretty(&self, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
		let indent = String::from_utf8(vec![32; level * 3]).unwrap();
		let _a = write!(f, "{}len      : {}\n", indent, self.len());
		let _b = write!(f, "{}prefix   : {:?}\n", indent, self.prefix);
		let _c = write!(f, "{}positions: {:?}\n", indent, self.positions);
		let _d = match self.left {
			None => write!(f, "{}left     : none\n", indent),
			Some(ref child) => {
				let _ = write!(f, "{}left (\n", indent);
				let _ = child.fmt_pretty(f, level + 1);
				write!(f, "{})\n", indent)
			}
		};
		match self.right {
			None => write!(f, "{}right    : none\n", indent),
			Some(ref child) => {
				let _ = write!(f, "{}right (\n", indent);
				let _ = child.fmt_pretty(f, level + 1);
				write!(f, "{})\n", indent)
			}
		}
	}

	pub fn serialize(&self, writer: &mut Write) -> bincode::Result<()> {
		serialize_into(writer, self)
	}

	pub fn deserialize(reader: &mut Read) -> bincode::Result<Self> {
		deserialize_from(reader)
	}
}

impl fmt::Debug for WaveletTrie {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.fmt_pretty(f, 0)
	}
}

mod tests;
