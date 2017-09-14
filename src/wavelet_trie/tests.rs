#[cfg(test)]
mod tests {
	use bit_vec_wrap::BitVecWrap;
	use wavelet_trie::WaveletTrie;

	#[test]
	fn insert_one_sequence() {
		let sequence = BitVecWrap::from_bytes(&[0b00001000]);
		let mut wt = WaveletTrie::new();
		wt.insert_static(&[sequence]);
		println!("{:?}", wt);
		// TODO: assert; wait for "rank"
	}

	#[test]
	fn insert_same_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00001000]);
		let mut wt = WaveletTrie::new();
		wt.insert_static(&[sequence1, sequence2]);
		println!("{:?}", wt);
		// TODO: assert; wait for "rank"
	}

	#[test]
	fn insert_two_different_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
		let mut wt = WaveletTrie::new();
		wt.insert_static(&[sequence1, sequence2]);
		println!("{:?}", wt);
		// TODO: assert; wait for "rank"
	}

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

	#[test]
	fn rank() {
		// this tests the binary example from the paper

		// 0001
		let mut s1 = BitVecWrap::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = BitVecWrap::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = BitVecWrap::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = BitVecWrap::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let s5 = s3.copy();
		let s6 = s4.copy();
		let s7 = s3.copy();

		let sequences = &[s1, s2, s3, s4, s5, s6, s7];
		let mut wt = WaveletTrie::new();
		wt.insert_static(sequences);
	}

}
