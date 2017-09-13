#[cfg(test)]
mod tests {
	use bit_vec_wrap::BitVecWrap;
	use wavelet_trie::WaveletTrie;

//	#[test]
//	fn insert_one_sequence() {
//		let sequence = BitVecWrap::from_bytes(&[0b00001000]);
//		let mut wt = WaveletTrie::new();
//		wt.insert_static(&[sequence]);
//		println!("{:?}", wt);
//		// TODO: assert; wait for "rank"
//	}
//
//	#[test]
//	fn insert_same_sequences() {
//		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
//		let sequence2 = BitVecWrap::from_bytes(&[0b00001000]);
//		let mut wt = WaveletTrie::new();
//		wt.insert_static(&[sequence1, sequence2]);
//		println!("{:?}", wt);
//		// TODO: assert; wait for "rank"
//	}
//
//	#[test]
//	fn insert_two_different_sequences() {
//		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
//		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
//		let mut wt = WaveletTrie::new();
//		wt.insert_static(&[sequence1, sequence2]);
//		println!("{:?}", wt);
//		// TODO: assert; wait for "rank"
//	}

	#[test]
	fn insert_different_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
		let sequence3 = BitVecWrap::from_bytes(&[0b00100001]);
		let mut wt = WaveletTrie::new();
		wt.insert_static(&[sequence1, sequence2, sequence3]);
		println!("{:?}", wt);
		// TODO: assert; wait for "rank"
	}
}
