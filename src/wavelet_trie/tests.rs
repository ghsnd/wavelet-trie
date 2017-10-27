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

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let mut wt = WaveletTrie::new();
		wt.insert_static(sequences);
		println!("{:?}", wt);

		assert_eq!(Some(0), wt.rank(&s3, 0));
		assert_eq!(Some(0), wt.rank(&s3, 2));
		assert_eq!(Some(1), wt.rank(&s3, 3));
		assert_eq!(Some(1), wt.rank(&s3, 4));
		assert_eq!(Some(2), wt.rank(&s3, 5));
		assert_eq!(Some(2), wt.rank(&s3, 6));
		assert_eq!(Some(3), wt.rank(&s3, 7));

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

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&s1, 0));
		assert_eq!(Ok(()), wt.insert(&s2, 1));
		assert_eq!(Ok(()), wt.insert(&s3, 2));
		assert_eq!(Ok(()), wt.insert(&s4, 3));
		assert_eq!(Ok(()), wt.insert(&s3, 4));
		assert_eq!(Ok(()), wt.insert(&s4, 5));
		assert_eq!(Ok(()), wt.insert(&s3, 6));
		assert_eq!(Ok(()), wt.insert(&s3, 7));
		
		println!("{:?}", wt);
		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
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

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert(&s1, 0));
		assert_eq!(Ok(()), wt.insert(&s3, 1));
		assert_eq!(Ok(()), wt.insert(&s3, 2));
		assert_eq!(Ok(()), wt.insert(&s3, 3));
		assert_eq!(Ok(()), wt.insert(&s4, 3));
		assert_eq!(Ok(()), wt.insert(&s4, 2));
		assert_eq!(Ok(()), wt.insert(&s2, 1));
		assert_eq!(Ok(()), wt.insert(&s3, 6));
		println!("{:?}", wt);
		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
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

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.append(&s1));
		assert_eq!(Ok(()), wt.append(&s2));
		assert_eq!(Ok(()), wt.append(&s3));
		assert_eq!(Ok(()), wt.append(&s4));
		assert_eq!(Ok(()), wt.append(&s3));
		assert_eq!(Ok(()), wt.append(&s4));
		assert_eq!(Ok(()), wt.append(&s3));
		assert_eq!(Ok(()), wt.append(&s3));

		println!("{:?}", wt);
		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
		assert_ranks(&wt, sequences);
	}

	#[test]
	fn access() {
		let sequence1 = BitVecWrap::from_bytes(&[0b00001000]);
		let sequence2 = BitVecWrap::from_bytes(&[0b00000001]);
		let sequence3 = BitVecWrap::from_bytes(&[0b00100001]);
		let sequences = &[sequence1.copy(), sequence2.copy(), sequence3.copy()];
		let wt = WaveletTrie::from_sequences(sequences);
		println!("{:?}", wt);
		assert_ranks(&wt, sequences);
		let pos_0_seq = wt.access(0);
		assert_eq!(sequence1, pos_0_seq);
		let pos_1_seq = wt.access(1);
		assert_eq!(sequence2, pos_1_seq);
		let pos_2_seq = wt.access(2);
		assert_eq!(sequence3, pos_2_seq);
	}

	#[test]
	fn select() {
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

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let wt = WaveletTrie::from_sequences(sequences);

		assert_eq!(Some(0), wt.select(&s1, 1));	// means: the first occurrence of s1 is at index 0!
		assert_eq!(None, wt.select(&s1, 2));	// there is no more s1 further in the trie
		assert_eq!(Some(1), wt.select(&s2, 1));	// s2 first occurs at index 1
		assert_eq!(Some(2), wt.select(&s3, 1));
		assert_eq!(Some(4), wt.select(&s3, 2));
		assert_eq!(Some(6), wt.select(&s3, 3));
		assert_eq!(None, wt.select(&s3, 4));
		assert_eq!(Some(3), wt.select(&s4, 1));
		assert_eq!(Some(5), wt.select(&s4, 2));
		assert_eq!(Some(2), wt.select(&s3, 1));
		assert_eq!(Some(4), wt.select(&s3, 2));
		assert_eq!(Some(6), wt.select(&s3, 3));
		assert_eq!(None, wt.select(&s3, 4));
		assert_eq!(Some(3), wt.select(&s4, 1));
		assert_eq!(Some(5), wt.select(&s4, 2));
		assert_eq!(Some(2), wt.select(&s3, 1));
		assert_eq!(Some(4), wt.select(&s3, 2));
		assert_eq!(Some(6), wt.select(&s3, 3));
		assert_eq!(None, wt.select(&s3, 4));
	}

		#[test]
	fn select_all() {
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

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let wt = WaveletTrie::from_sequences(sequences);

		assert_eq!(vec![0], wt.select_all(&s1));
		assert_eq!(vec![1], wt.select_all(&s2));
		assert_eq!(vec![2, 4, 6], wt.select_all(&s3));
		assert_eq!(vec![3, 5], wt.select_all(&s4));
		let empty_vec: Vec<usize> = Vec::new();
		assert_eq!(empty_vec, wt.select_all(&BitVecWrap::from_elem(16, true))); //not in trie

		let mut existing_prefix = BitVecWrap::new();
		existing_prefix.push(false);
		existing_prefix.push(false);
		assert_eq!(vec![0, 1, 3, 5], wt.select_all(&existing_prefix));

		let prefix_zero = BitVecWrap::from_elem(0, false);
		assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], wt.select_all(&prefix_zero));
	}

	#[test]
	fn append_str() {
		let mut wt = WaveletTrie::new();
		wt.append_str("Dit is een test");
		wt.append_str("Dit is een teletubbie");
		println!("{:?}", wt);
	}
}
