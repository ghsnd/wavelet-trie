extern crate bit_vec;
use self::bit_vec::BitVec;

// This is a wrapper around BitVec to implement methods not supported
// directly by the bit_vec crate, in a very naive way.
// TODO In the ideal case, this is replaced by a
// "dynamic bitvector with indels", i.e. bits can be inserted or deleted
// at arbitrary points in the vector. It can even be compressed! See
// V. Mäkinen and G. Navarro. Dynamic entropy-compressed sequences and full-text indexes.

#[derive(Clone, PartialEq, Debug)]
pub struct BitVecWrap {
	bit_vec: BitVec,
}

impl BitVecWrap {

	// constructor
	pub fn new() -> Self {
		BitVecWrap {
			bit_vec: BitVec::new()
		}
	}

	// constructor
	pub fn from_elem(nbits: usize, bit: bool) -> Self {
		BitVecWrap {
			bit_vec: BitVec::from_elem(nbits, bit)
		}
	}

	// constructor
	pub fn from_bytes(bytes: &[u8]) -> Self {
		BitVecWrap {
			bit_vec: BitVec::from_bytes(bytes)
		}
	}

	pub fn get(&self, i: usize) -> Option<bool> {
		self.bit_vec.get(i)
	}

	// set a bit at index i
	pub fn set(&mut self, i: usize, elem: bool) {
		self.bit_vec.set(i, elem);
	}

	// add a bit at the end
	pub fn push(&mut self, elem: bool) {
		self.bit_vec.push(elem);
	}

	// remove the last bit and returns it. Returns None if the bitvector is empty.
	pub fn pop(&mut self) -> Option<bool> {
		self.bit_vec.pop()
	}

	// insert a bit at index i, hereby shifting the bits after i one position towards the end
	// OPTIMIZEME
	pub fn insert(&mut self, i: usize, elem: bool) {
		self.bit_vec.push(false); // just to make it grow
		for index in (i..self.bit_vec.len()).rev() {
			let prev_bit = self.bit_vec.get(index - 1);
			if let Some(bit) = prev_bit {
				self.bit_vec.set(index, bit);
			}
		}
		self.bit_vec.set(i, elem);
	}

	/*pub fn insert(&mut self, i: usize, other: &BitVecWrap) {
		
	}*/

	// delete a bit at index i, hereby shifting the bits after i one position towards the beginning
	// OPTIMIZEME
	pub fn delete(&mut self, i: usize) {
		for index in (i + 1)..self.bit_vec.len() {
			let next_bit = self.bit_vec.get(index);
			if let Some(bit) = next_bit {
				self.bit_vec.set(index - 1, bit);
			}
		}
		self.bit_vec.pop();
	}

	// Number of ones in the vector before position "pos"
	// i.e. in [0 .. pos-1]
	fn rank_one(&self, pos: usize) -> usize {
		if pos > self.bit_vec.len() {
			panic!("Index out of bounds!");
		}
		let block_iter = self.bit_vec.blocks();
		let low_pos = pos / 32; // 1 block = u32
		let low_pos_rem = pos % 32;

		// first count 1-bits up to low_pos
		let mut bit_count = block_iter.take(low_pos).fold(0, |nr_bits, block| nr_bits + block.count_ones() as usize);

		// now count the remaining bits up to the real position
		let start_pos = pos - low_pos_rem;
		for bit_pos in start_pos..pos {
			match self.bit_vec.get(bit_pos) {
				Some(true) => bit_count += 1,
				_ => {}
			}
		}
		bit_count
	}

	fn rank_zero(&self, pos: usize) -> usize {
		if pos == 0 {
			pos
		} else {
			pos - self.rank_one(pos)
		}
	}

	pub fn is_empty(&self) -> bool {
		self.bit_vec.is_empty()
	}

	pub fn len(&self) -> usize {
		self.bit_vec.len()
	}

	pub fn truncate(&mut self, len: usize) {
		self.bit_vec.truncate(len);
	}

	// get the <common prefix> part of <common prefix><different bit><different suffix>
	// as defined in
	// R. Grossi, G. Ottoviano "The Wavelet Trie: Maintaining an Indexed Sequence of Strings in Compressed Space"
	// which is not what one might expect in the case of equal bitvectors!
	pub fn longest_common_prefix (&self, other: &BitVecWrap) -> BitVecWrap {
		if self == other {
			let mut bit_vec_clone = self.clone();
			//bit_vec_clone.pop();
			//bit_vec_clone.pop();
			bit_vec_clone.truncate(self.len() - 2);
			bit_vec_clone
		} else {
			// OPTIMIZEME
			let mut new_bit_vec = BitVecWrap::new();
			let mut done = false;
			let mut index = 0;
			while index < self.len() && index < other.len() && !done {
				if let Some(bit_one) = self.get(index) {
					if let Some(bit_two) = other.get(index) {
						if bit_one == bit_two {
							new_bit_vec.push(bit_one);
						} else {
							done = true;
						}
					}
				}
				index = index + 1;
			}
			new_bit_vec
		}
	}

	pub fn different_suffix(&self, lcp: &BitVecWrap) -> (bool, BitVecWrap) {
		let first_bit = self.get(lcp.len()).unwrap();
		let mut new_bitvector = BitVecWrap::new();
		new_bitvector.bit_vec = self.bit_vec.iter().skip(lcp.len() + 1).collect();
		(first_bit, new_bitvector)
	}

}

#[cfg(test)]
mod tests {
	use bit_vec_wrap::BitVecWrap;

	#[test]
	fn rank_one() {
		let mut bv = BitVecWrap::from_elem(75, false);
		bv.set(4, true);
		bv.set(5, true);
		bv.set(35, true);
		bv.set(74, true);
		assert_eq!(0, bv.rank_one(0));
		assert_eq!(0, bv.rank_one(4));
		assert_eq!(1, bv.rank_one(5));
		assert_eq!(2, bv.rank_one(6));
		assert_eq!(2, bv.rank_one(33));
		assert_eq!(3, bv.rank_one(36));
		assert_eq!(4, bv.rank_one(75));
	}

	#[test]
	fn rank_zero() {
		let mut bv = BitVecWrap::from_elem(75, true);
		bv.set(4, false);
		bv.set(5, false);
		bv.set(35, false);
		bv.set(74, false);
		assert_eq!(0, bv.rank_zero(0));
		assert_eq!(0, bv.rank_zero(4));
		assert_eq!(1, bv.rank_zero(5));
		assert_eq!(2, bv.rank_zero(6));
		assert_eq!(2, bv.rank_zero(33));
		assert_eq!(3, bv.rank_zero(36));
		assert_eq!(4, bv.rank_zero(75));
	}

	#[test]
	fn insert() {
		let mut bv = BitVecWrap::new();
		bv.push(false);
		bv.push(false);		// bv = [0,0]
		bv.insert(1, true);
		assert_eq!(false, bv.get(0).unwrap());
		assert_eq!(true, bv.get(1).unwrap());
		assert_eq!(false, bv.get(2).unwrap());
	}

	#[test]
	fn delete() {
		let mut bv = BitVecWrap::new();
		bv.push(true);
		bv.push(false);
		bv.push(true);
		bv.delete(1);
		assert_eq!(true, bv.get(0).unwrap());
		assert_eq!(true, bv.get(1).unwrap());
	}

	#[test]
	fn longest_common_prefix_equal() {
		let bv1 = BitVecWrap::from_bytes(&[0b01010101]);
		let bv2 = BitVecWrap::from_bytes(&[0b01010101]);
		let longest_common_prefix = bv1.longest_common_prefix(&bv2);
		let mut should_be_prefix = BitVecWrap::from_bytes(&[0b01010101]);
		should_be_prefix.pop();
		should_be_prefix.pop();
		assert_eq!(should_be_prefix, longest_common_prefix);
	}

	#[test]
	fn longest_common_prefix_different() {
		let bv1 = BitVecWrap::from_bytes(&[0b01010101, 0b01010101]);
		let bv2 = BitVecWrap::from_bytes(&[0b01010101, 0b01011101, 0b00011101]);
		let longest_common_prefix = bv1.longest_common_prefix(&bv2);
		let mut should_be_prefix = BitVecWrap::from_bytes(&[0b01010101, 0b01011101]);
		should_be_prefix.pop();
		should_be_prefix.pop();
		should_be_prefix.pop();
		should_be_prefix.pop();
		assert_eq!(should_be_prefix, longest_common_prefix);
	}

	#[test]
	fn longest_common_prefix_empty() {
		let bv1 = BitVecWrap::new();
		let bv2 = BitVecWrap::from_bytes(&[0b01010101]);
		let longest_common_prefix = bv1.longest_common_prefix(&bv2);
		assert_eq!(0, longest_common_prefix.len());
	}

	#[test]
	fn different_suffix() {
		let bv1     = BitVecWrap::from_bytes(&[0b01010101, 0b01010101]);
		let mut bv2 = BitVecWrap::from_bytes(&[0b01010101, 0b01011101, 0b00011101]);
		let longest_common_prefix = bv1.longest_common_prefix(&bv2);
		let mut should_be_suffix = BitVecWrap::from_bytes(&[0b10100011]);
		should_be_suffix.push(true);
		should_be_suffix.push(false);
		should_be_suffix.push(true);
		let result: (bool, BitVecWrap) = bv2.different_suffix(&longest_common_prefix);
		assert_eq!(should_be_suffix, result.1);
		assert_eq!(true, result.0);
	}
}
