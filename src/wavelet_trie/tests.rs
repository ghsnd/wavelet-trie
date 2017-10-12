#[cfg(test)]
mod tests {
	use bit_vec_wrap::BitVecWrap;
	use wavelet_trie::WaveletTrie;
	use std::collections::HashMap;

	// inserts the sequences statically in a wavelet trie and checks the
	// ranks of the sequences at the last position when inserted.
	fn insert_static_and_check(sequences: &[BitVecWrap]) {
		let wt = WaveletTrie::from_sequences(sequences);
		assert_ranks(&wt, sequences);
	}

	fn assert_ranks(wt: &WaveletTrie, sequences: &[BitVecWrap]) {
		let len = wt.len();
		let mut sequence_counter = HashMap::new();
		for sequence in sequences {
			let counter = sequence_counter.entry(sequence).or_insert(0);
			*counter += 1;
		}
		for (sequence, count) in sequence_counter {
			assert_eq!(Some(count), wt.rank(sequence, len));
		}
	}

	#[test]
	fn insert_one_sequence() {
		let sequence = BitVecWrap::from_bytes(&[0b00001000]);
		insert_static_and_check(&[sequence]);
	}

	#[test]
	fn insert_same_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00001000]);
		insert_static_and_check(&[sequence1, sequence2]);
	}

	#[test]
	fn insert_two_different_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
		insert_static_and_check(&[sequence1, sequence2]);
	}

	#[test]
	fn insert_different_sequences() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
		let sequence3 = BitVecWrap::from_bytes(&[0b00100001]);
		insert_static_and_check(&[sequence1, sequence2, sequence3]);
	}

	#[test]
	fn rank() {
		// this tests the binary example from the paper
		// see also example.txt in de root of this repo

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
		let s8 = s3.copy();

		let sequences = &[s1, s2, s3, s4, s5, s6, s7];
		let mut wt = WaveletTrie::new();
		wt.insert_static(sequences);
		println!("{:?}", wt);

		assert_eq!(Some(0), wt.rank(&s8, 0));
		assert_eq!(Some(0), wt.rank(&s8, 2));
		assert_eq!(Some(1), wt.rank(&s8, 3));
		assert_eq!(Some(1), wt.rank(&s8, 4));
		assert_eq!(Some(2), wt.rank(&s8, 5));
		assert_eq!(Some(2), wt.rank(&s8, 6));
		assert_eq!(Some(3), wt.rank(&s8, 7));

		let mut seq_0 = BitVecWrap::new();
		seq_0.push(false);
		for number in 0..7 {
			assert_eq!(Some(number), wt.rank(&seq_0, number));
		}

		let mut seq_none = BitVecWrap::new();
		seq_none.push(false);
		seq_none.push(false);
		seq_none.push(false);
		seq_none.push(false);
		assert_eq!(None, wt.rank(&seq_none, 7));
	}

	#[test]
	fn insert_dynamic_one_sequence() {
		let sequence = BitVecWrap::from_bytes(&[0b00001000]);
		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&sequence, 0));
		assert_ranks(&wt, &[sequence])
	}

	#[test]
	fn insert_dynamic_out_of_order() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence3 = BitVecWrap::from_bytes(&[0b00011000]);
		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&sequence1, 0));
		assert_eq!(Ok(()), wt.insert(&sequence2, 1));
		assert_eq!(Ok(()), wt.insert(&sequence3, 0));
		let sequences = &[sequence1, sequence2, sequence3];
		assert_ranks(&wt, sequences);
	}

	#[test]
	fn insert_example_dynamic_in_order() {
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
		let s8 = s3.copy();

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&s1, 0));
		assert_eq!(Ok(()), wt.insert(&s2, 1));
		assert_eq!(Ok(()), wt.insert(&s3, 2));
		assert_eq!(Ok(()), wt.insert(&s4, 3));
		assert_eq!(Ok(()), wt.insert(&s5, 4));
		assert_eq!(Ok(()), wt.insert(&s6, 5));
		assert_eq!(Ok(()), wt.insert(&s7, 6));
		assert_eq!(Ok(()), wt.insert(&s8, 7));
		
		println!("{:?}", wt);
		let sequences = &[s1, s2, s3, s4, s5, s6, s7, s8];
		assert_ranks(&wt, sequences);
	}

	#[test]
	fn insert_example_dynamic_out_of_order() {
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
		let s8 = s3.copy();

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&s1, 0));
		assert_eq!(Ok(()), wt.insert(&s3, 1));
		assert_eq!(Ok(()), wt.insert(&s5, 2));
		assert_eq!(Ok(()), wt.insert(&s8, 3));
		assert_eq!(Ok(()), wt.insert(&s6, 3));
		assert_eq!(Ok(()), wt.insert(&s4, 2));
		assert_eq!(Ok(()), wt.insert(&s2, 1));
		assert_eq!(Ok(()), wt.insert(&s7, 6));
		println!("{:?}", wt);
		let sequences = &[s1, s2, s3, s4, s5, s6, s7, s8];
		assert_ranks(&wt, sequences);
	}

	#[test]
	fn append() {
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
		let s8 = s3.copy();

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.append(&s1));
		assert_eq!(Ok(()), wt.append(&s2));
		assert_eq!(Ok(()), wt.append(&s3));
		assert_eq!(Ok(()), wt.append(&s4));
		assert_eq!(Ok(()), wt.append(&s5));
		assert_eq!(Ok(()), wt.append(&s6));
		assert_eq!(Ok(()), wt.append(&s7));
		assert_eq!(Ok(()), wt.append(&s8));

		println!("{:?}", wt);
		let sequences = &[s1, s2, s3, s4, s5, s6, s7, s8];
		assert_ranks(&wt, sequences);
	}

}
