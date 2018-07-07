#[cfg(test)]
mod tests {
	extern crate dyn_bit_vec;
	use self::dyn_bit_vec::DBVec;
	use wavelet_trie::WaveletTrie;
	use std::collections::HashMap;

	// inserts the sequences statically in a wavelet trie and checks the
	// ranks of the sequences at the last position when inserted.
	fn insert_static_and_check_d(sequences: &[DBVec]) {
		let wt = WaveletTrie::from_sequences_d(sequences);
		println!("{:?}", wt);
		assert_ranks_d(&wt, sequences);
	}

	fn assert_ranks_d(wt: &WaveletTrie, sequences: &[DBVec]) {
		let len = wt.len_d();
		let mut sequence_counter = HashMap::new();
		for sequence in sequences {
			let counter = sequence_counter.entry(sequence).or_insert(0);
			*counter += 1;
		}
		for (sequence, count) in sequence_counter {
			assert_eq!(Some(count), wt.rank_d(sequence, len));
		}
	}

	#[test]
	fn insert_static_d() {
		let sequence = DBVec::from_bytes(&[0b00001000]);
		insert_static_and_check_d(&[sequence]);
	}

	#[test]
	fn insert_same_sequences_d() {
		let sequence1 = DBVec::from_bytes(&[0b00001000]);
		let sequence2 = DBVec::from_bytes(&[0b00001000]);
		insert_static_and_check_d(&[sequence1, sequence2]);
	}

	#[test]
	fn insert_one_sequence_d() {
		let sequence = DBVec::from_bytes(&[0b00001000]);
		insert_static_and_check_d(&[sequence]);
	}

	#[test]
	fn insert_two_different_sequences_d() {
		let sequence1 = DBVec::from_bytes(&[0b00001000]);
		let sequence2 = DBVec::from_bytes(&[0b10000000]);
		insert_static_and_check_d(&[sequence1, sequence2]);
	}

	#[test]
	fn insert_fails_dynamic_staticcase() {
		let sequence1 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b01110010011011110010111000110011, 0b00111001001100010010111101100111, 0b00110000001011110011100100111001, 0b00110010001100100010111100110010, 0b01100110011001000111001000101101, 0b01101110011110010111001100101101, 0b00101101011110000110000101110100, 0b01110100001000110111001101101110, 0b00111110011001010111000001111001, 0b01110100011010000011110000100000, 0b00101111001110100111000001110100, 0b01110111011101110111011100101111, 0b00101110001100110111011100101110, 0b00101111011001110111001001101111, 0b00110010001100000011000000110010, 0b00101111001101110011000000101111, 0b00100011011011000111011101101111, 0b01101111011101000110111001001111, 0b01111001011001110110111101101100, 0b00000000001011100010000000111110]);
		let sequence2 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b01110010011011110010111000110011, 0b00111001001100010010111101100111, 0b00110000001011110011100100111001, 0b00110010001100100010111100110010, 0b01100110011001000111001000101101, 0b01101110011110010111001100101101, 0b00101101011110000110000101110100, 0b01110100001000110111001101101110, 0b00111110011001010111000001111001, 0b01110100011010000011110000100000, 0b00101111001110100111000001110100, 0b01110010011101010111000000101111, 0b01110010011011110010111001101100, 0b01101111011101100010111101100111, 0b01101101011011010110111101100011, 0b00101111011100110110111001101111, 0b01100110011000010110111101110110, 0b01100011011011110101011000100011, 0b01101100011101010110001001100001, 0b00111110011110010111001001100001, 0b00000000000000000010111000100000]);
		let sequence3 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110000001011110010111100111010, 0b00101110011011000111001001110101, 0b00101111011001110111001001101111, 0b01100001011000110110111101110110, 0b01100001011101100010111101100010, 0b01110000001011110110111001101110, 0b01100101011001100110010101110010, 0b01100100011001010111001001110010, 0b01100101011011010110000101001110, 0b01100011011000010111000001110011, 0b01100101011100100101000001100101, 0b00111110011110000110100101100110, 0b01100010011001000010001000100000, 0b00101110001000000010001001101111, 0b00000000000000000000000000000000]);
		let sequence4 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110000001011110010111100111010, 0b00101110011011000111001001110101, 0b00101111011001110111001001101111, 0b01100001011000110110111101110110, 0b01100001011101100010111101100010, 0b01110000001011110110111001101110, 0b01100101011001100110010101110010, 0b01100100011001010111001001110010, 0b01100101011011010110000101001110, 0b01100011011000010111000001110011, 0b01101001011100100101010101100101, 0b01101000001000100010000000111110, 0b00111010011100000111010001110100, 0b01100010011001000010111100101111, 0b01101001011001000110010101110000, 0b01110010011011110010111001100001, 0b01101110011011110010111101100111, 0b01101111011011000110111101110100, 0b00100010001011110111100101100111, 0b00000000000000000010111000100000]);
		insert_static_and_check_d(&[sequence1, sequence2, sequence3, sequence4]);
	}

	#[test]
	fn insert_fails_dynamic_dynamiccase_long() {
		let sequence1 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b01110010011011110010111000110011, 0b00111001001100010010111101100111, 0b00110000001011110011100100111001, 0b00110010001100100010111100110010, 0b01100110011001000111001000101101, 0b01101110011110010111001100101101, 0b00101101011110000110000101110100, 0b01110100001000110111001101101110, 0b00111110011001010111000001111001, 0b01110100011010000011110000100000, 0b00101111001110100111000001110100, 0b01110111011101110111011100101111, 0b00101110001100110111011100101110, 0b00101111011001110111001001101111, 0b00110010001100000011000000110010, 0b00101111001101110011000000101111, 0b00100011011011000111011101101111, 0b01101111011101000110111001001111, 0b01111001011001110110111101101100, 0b00000000001011100010000000111110]);
		let sequence2 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b01110010011011110010111000110011, 0b00111001001100010010111101100111, 0b00110000001011110011100100111001, 0b00110010001100100010111100110010, 0b01100110011001000111001000101101, 0b01101110011110010111001100101101, 0b00101101011110000110000101110100, 0b01110100001000110111001101101110, 0b00111110011001010111000001111001, 0b01110100011010000011110000100000, 0b00101111001110100111000001110100, 0b01110010011101010111000000101111, 0b01110010011011110010111001101100, 0b01101111011101100010111101100111, 0b01101101011011010110111101100011, 0b00101111011100110110111001101111, 0b01100110011000010110111101110110, 0b01100011011011110101011000100011, 0b01101100011101010110001001100001, 0b00111110011110010111001001100001, 0b00000000000000000010111000100000]);
		let sequence3 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110000001011110010111100111010, 0b00101110011011000111001001110101, 0b00101111011001110111001001101111, 0b01100001011000110110111101110110, 0b01100001011101100010111101100010, 0b01110000001011110110111001101110, 0b01100101011001100110010101110010, 0b01100100011001010111001001110010, 0b01100101011011010110000101001110, 0b01100011011000010111000001110011, 0b01100101011100100101000001100101, 0b00111110011110000110100101100110, 0b01100010011001000010001000100000, 0b00101110001000000010001001101111, 0b00000000000000000000000000000000]);
		let sequence4 = DBVec::from_u32_slice(&[0b01110100011101000110100000111100, 0b00101111001011110011101001110000, 0b01100101011100000110001001100100, 0b00101110011000010110100101100100, 0b00101111011001110111001001101111, 0b01101111011101000110111001101111, 0b01111001011001110110111101101100, 0b00111100001000000011111000101111, 0b01110000011101000111010001101000, 0b01110000001011110010111100111010, 0b00101110011011000111001001110101, 0b00101111011001110111001001101111, 0b01100001011000110110111101110110, 0b01100001011101100010111101100010, 0b01110000001011110110111001101110, 0b01100101011001100110010101110010, 0b01100100011001010111001001110010, 0b01100101011011010110000101001110, 0b01100011011000010111000001110011, 0b01101001011100100101010101100101, 0b01101000001000100010000000111110, 0b00111010011100000111010001110100, 0b01100010011001000010111100101111, 0b01101001011001000110010101110000, 0b01110010011011110010111001100001, 0b01101110011011110010111101100111, 0b01101111011011000110111101110100, 0b00100010001011110111100101100111, 0b00000000000000000010111000100000]);
		let mut wt = WaveletTrie::new();
		println!("*********\n{:?}\n*********", wt);
		wt.append_d(&sequence1);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence2);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence3);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence4);
		println!("\n*********\n{:?}\n*********", wt);

		// test ranks
		assert_eq!(Some(1), wt.rank_d(&sequence1, 1));
		assert_eq!(Some(1), wt.rank_d(&sequence2, 2));
		assert_eq!(Some(1), wt.rank_d(&sequence3, 3));
		assert_eq!(Some(1), wt.rank_d(&sequence4, 4));
	}

	#[test]
	fn insert_fails_dynamic_dynamiccase_short() {
		let sequence1 = DBVec::from_u32_slice(&[0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b00000000001011100010000000111110]);
		let sequence2 = DBVec::from_u32_slice(&[0b01110111001011110010111100111010, 0b01110111001011100111011101110111, 0b00111110011110010111001001100001, 0b00000000000000000010111000100000]);
		let sequence3 = DBVec::from_u32_slice(&[0b01110000001011110010111100111010, 0b00101110001000000010001001101111, 0b00000000000000000000000000000000]);
		let sequence4 = DBVec::from_u32_slice(&[0b01110000001011110010111100111010, 0b01100010011001000010111100101111, 0b01101001011001000110010101110000, 0b01110010011011110010111001100001, 0b01101110011011110010111101100111, 0b01101111011011000110111101110100, 0b00100010001011110111100101100111, 0b00000000000000000010111000100000]);
		let mut wt = WaveletTrie::new();
		println!("*********\n{:?}\n*********", wt);
		wt.append_d(&sequence1);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence2);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence3);
		println!("\n*********\n{:?}\n*********", wt);
		wt.append_d(&sequence4);
		println!("\n*********\n{:?}\n*********", wt);

		// test ranks
		assert_eq!(Some(1), wt.rank_d(&sequence1, 1));
		assert_eq!(Some(1), wt.rank_d(&sequence2, 2));
		assert_eq!(Some(1), wt.rank_d(&sequence3, 3));
		assert_eq!(Some(1), wt.rank_d(&sequence4, 4));
	}

	#[test]
	fn rank_d() {
		// this tests the binary example from the paper
		// see also example.txt in de root of this repo

		// 1000
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 1100
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let wt = WaveletTrie::from_sequences_d(sequences);
		println!("{:?}", wt);

		assert_eq!(Some(0), wt.rank_d(&s3, 0));
		assert_eq!(Some(0), wt.rank_d(&s3, 2));
		assert_eq!(Some(1), wt.rank_d(&s3, 3));
		assert_eq!(Some(1), wt.rank_d(&s3, 4));
		assert_eq!(Some(2), wt.rank_d(&s3, 5));
		assert_eq!(Some(2), wt.rank_d(&s3, 6));
		assert_eq!(Some(3), wt.rank_d(&s3, 7));

		let mut seq_0 = DBVec::new();
		seq_0.push(false);
		for number in 0..7 {
			assert_eq!(Some(number), wt.rank_d(&seq_0, number));
		}

		let mut seq_none = DBVec::new();
		seq_none.push(false);
		seq_none.push(false);
		seq_none.push(false);
		seq_none.push(false);
		assert_eq!(None, wt.rank_d(&seq_none, 7));
	}

	#[test]
	fn insert_dynamic_one_sequence_d() {
		let sequence = DBVec::from_bytes(&[0b00001000]);
		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert_d(&sequence, 0));
		assert_ranks_d(&wt, &[sequence])
	}

	#[test]
	fn insert_dynamic_out_of_order_d() {
		let sequence1 = DBVec::from_bytes(&[0b00001000]);
		let sequence2 = DBVec::from_bytes(&[0b00001000]);
		let sequence3 = DBVec::from_bytes(&[0b00011000]);
		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert_d(&sequence1, 0));
		assert_eq!(Ok(()), wt.insert_d(&sequence2, 1));
		assert_eq!(Ok(()), wt.insert_d(&sequence3, 0));
		let sequences = &[sequence1, sequence2, sequence3];
		assert_ranks_d(&wt, sequences);
	}

	#[test]
	fn insert_example_dynamic_in_order_d() {
		// 0001
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert_d(&s1, 0));
		assert_eq!(Ok(()), wt.insert_d(&s2, 1));
		assert_eq!(Ok(()), wt.insert_d(&s3, 2));
		assert_eq!(Ok(()), wt.insert_d(&s4, 3));
		assert_eq!(Ok(()), wt.insert_d(&s3, 4));
		assert_eq!(Ok(()), wt.insert_d(&s4, 5));
		assert_eq!(Ok(()), wt.insert_d(&s3, 6));
		assert_eq!(Ok(()), wt.insert_d(&s3, 7));

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
		println!("{:?}", wt);
		assert_ranks_d(&wt, sequences);
		wt.print_stats_d();
	}

	#[test]
	fn insert_example_dynamic_out_of_order_d() {
		// 0001
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.insert_d(&s1, 0));
		assert_eq!(Ok(()), wt.insert_d(&s3, 1));
		assert_eq!(Ok(()), wt.insert_d(&s3, 2));
		assert_eq!(Ok(()), wt.insert_d(&s3, 3));
		assert_eq!(Ok(()), wt.insert_d(&s4, 3));
		assert_eq!(Ok(()), wt.insert_d(&s4, 2));
		assert_eq!(Ok(()), wt.insert_d(&s2, 1));
		assert_eq!(Ok(()), wt.insert_d(&s3, 6));
		println!("{:?}", wt);
		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
		assert_ranks_d(&wt, sequences);
	}

	#[test]
	fn append_d() {
		// 0001
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.append_d(&s1));
		assert_eq!(Ok(()), wt.append_d(&s2));
		assert_eq!(Ok(()), wt.append_d(&s3));
		assert_eq!(Ok(()), wt.append_d(&s4));
		assert_eq!(Ok(()), wt.append_d(&s3));
		assert_eq!(Ok(()), wt.append_d(&s4));
		assert_eq!(Ok(()), wt.append_d(&s3));
		assert_eq!(Ok(()), wt.append_d(&s3));

		println!("{:?}", wt);
		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy(), s3.copy()];
		assert_ranks_d(&wt, sequences);
	}

	#[test]
	fn access_d() {
		let sequence1 = DBVec::from_bytes(&[0b00010000]);
		let sequence2 = DBVec::from_bytes(&[0b10000000]);
		let sequence3 = DBVec::from_bytes(&[0b10000100]);
		let sequences = &[sequence1.copy(), sequence2.copy(), sequence3.copy()];
		let wt = WaveletTrie::from_sequences_d(sequences);
		println!("{:?}", wt);
		assert_ranks_d(&wt, sequences);
		let pos_0_seq = wt.access_d(0);
		assert_eq!(sequence1, pos_0_seq);
		let pos_1_seq = wt.access_d(1);
		assert_eq!(sequence2, pos_1_seq);
		let pos_2_seq = wt.access_d(2);
		assert_eq!(sequence3, pos_2_seq);
	}

	#[test]
	fn select_d() {
		// 0001
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let wt = WaveletTrie::from_sequences_d(sequences);

		assert_eq!(Some(0), wt.select_d(&s1, 1));	// means: the first occurrence of s1 is at index 0!
		assert_eq!(None, wt.select_d(&s1, 2));	// there is no more s1 further in the trie
		assert_eq!(Some(1), wt.select_d(&s2, 1));	// s2 first occurs at index 1
		assert_eq!(Some(2), wt.select_d(&s3, 1));
		assert_eq!(Some(4), wt.select_d(&s3, 2));
		assert_eq!(Some(6), wt.select_d(&s3, 3));
		assert_eq!(None, wt.select_d(&s3, 4));
		assert_eq!(Some(3), wt.select_d(&s4, 1));
		assert_eq!(Some(5), wt.select_d(&s4, 2));
		assert_eq!(Some(2), wt.select_d(&s3, 1));
		assert_eq!(Some(4), wt.select_d(&s3, 2));
		assert_eq!(Some(6), wt.select_d(&s3, 3));
		assert_eq!(None, wt.select_d(&s3, 4));
		assert_eq!(Some(3), wt.select_d(&s4, 1));
		assert_eq!(Some(5), wt.select_d(&s4, 2));
		assert_eq!(Some(2), wt.select_d(&s3, 1));
		assert_eq!(Some(4), wt.select_d(&s3, 2));
		assert_eq!(Some(6), wt.select_d(&s3, 3));
		assert_eq!(None, wt.select_d(&s3, 4));
	}

	#[test]
	fn select_all_d() {
		// 0001
		let mut s1 = DBVec::new();
		s1.push(false);
		s1.push(false);
		s1.push(false);
		s1.push(true);

		// 0011
		let mut s2 = DBVec::new();
		s2.push(false);
		s2.push(false);
		s2.push(true);
		s2.push(true);

		// 0100
		let mut s3 = DBVec::new();
		s3.push(false);
		s3.push(true);
		s3.push(false);
		s3.push(false);

		// 00100
		let mut s4 = DBVec::new();
		s4.push(false);
		s4.push(false);
		s4.push(true);
		s4.push(false);
		s4.push(false);

		let sequences = &[s1.copy(), s2.copy(), s3.copy(), s4.copy(), s3.copy(), s4.copy(), s3.copy()];
		let wt = WaveletTrie::from_sequences_d(sequences);

		assert_eq!(vec![0], wt.select_all_d(&s1));
		assert_eq!(vec![1], wt.select_all_d(&s2));
		assert_eq!(vec![2, 4, 6], wt.select_all_d(&s3));
		assert_eq!(vec![3, 5], wt.select_all_d(&s4));
		let empty_vec: Vec<u64> = Vec::new();
		assert_eq!(empty_vec, wt.select_all_d(&DBVec::from_elem(16, true))); //not in trie

		let mut existing_prefix = DBVec::new();
		existing_prefix.push(false);
		existing_prefix.push(false);
		assert_eq!(vec![0, 1, 3, 5], wt.select_all_d(&existing_prefix));

		let prefix_zero = DBVec::from_elem(0, false);
		assert_eq!(vec![0, 1, 2, 3, 4, 5, 6], wt.select_all_d(&prefix_zero));
	}

	#[test]
	fn str_ops_d() {
		let mut wt = WaveletTrie::new();
		assert_eq!(Ok(()), wt.append_str_d("Dit is een test"));
		assert_eq!(Ok(()), wt.append_str_d("Dit is een teletubbie"));
		//println!("{:?}", wt);
		assert_eq!(Some(2), wt.rank_str_d("Dit is", 2));
		assert_eq!(None, wt.rank_str_d("st", 2));
		assert_eq!(Some(1), wt.rank_str_d("Dit is een tele", 2));
		assert_eq!(String::from("Dit is een test"), wt.access_str_d(0).unwrap());
		assert_eq!(String::from("Dit is een teletubbie"), wt.access_str_d(1).unwrap());
		assert_eq!(Some(0), wt.select_str_d("Dit is een test", 1));
		assert_eq!(Some(1), wt.select_str_d("Dit is een teletubbie", 1));
		assert_eq!(Some(1), wt.select_str_d("Dit is een te", 2));
		assert_eq!(vec![0, 1], wt.select_all_str_d("Dit is een"));
		wt.print_stats_d();
	}

	#[test]
	fn access_str() {
		let mut wt = WaveletTrie::new();
		let str_vec = vec!(
			"<http://dbpedia.org/ontology/> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Ontology> .",
			"<http://dbpedia.org/ontology/> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://purl.org/vocommons/voaf#Vocabulary> .",
			"<http://dbpedia.org/ontology/> <http://purl.org/vocab/vann/preferredNamespacePrefix> \"dbo\" .",
			"<http://dbpedia.org/ontology/> <http://purl.org/vocab/vann/preferredNamespaceUri> \"http://dbpedia.org/ontology/\" .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/title> \"The DBpedia Ontology\"@en .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/description> \"\\n              The DBpedia ontology provides the classes and properties used in the DBpedia data set.\\n            \"@en .",
			"<http://dbpedia.org/ontology/> <http://xmlns.com/foaf/0.1/homepage> <http://wiki.dbpedia.org/Ontology> .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/source> <http://mappings.dbpedia.org> .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/publisher> \"DBpedia Maintainers\" .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/creator> \"DBpedia Maintainers and Contributors\" .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/issued> \"2008-11-17T12:00Z\" .",
			"<http://dbpedia.org/ontology/> <http://purl.org/dc/terms/modified> \"2017-01-16T16:04Z\" .",
			"<http://dbpedia.org/ontology/> <http://www.w3.org/2002/07/owl#versionInfo> \"4.2-SNAPSHOT\"@en .",
			"<http://dbpedia.org/ontology/> <http://www.w3.org/2000/01/rdf-schema#comment> \"\\n              This ontology is generated from the manually created specifications in the DBpedia Mappings\\n              Wiki. Each release of this ontology corresponds to a new release of the DBpedia data set which\\n              contains instance data extracted from the different language versions of Wikipedia. For\\n              information regarding changes in this ontology, please refer to the DBpedia Mappings Wiki.\\n            \"@en .",
			"<http://dbpedia.org/ontology/> <http://creativecommons.org/ns#license> <http://www.gnu.org/copyleft/fdl.html> .",
			"<http://dbpedia.org/ontology/> <http://creativecommons.org/ns#license> <http://creativecommons.org/licenses/by-sa/3.0/> .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"Basketball-Liga\"@de .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"basketball league\"@en .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"Ομοσπονδία Καλαθοσφαίρισης\"@el .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"バスケットボールリーグ\"@ja .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"basketbal competitie\"@nl .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"lega di pallacanestro\"@it .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"농구 리그\"@ko .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"ligue de basketball\"@fr .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"sraith cispheile\"@ga .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#label> \"liga de baloncesto\"@es .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#comment> \"a group of sports teams that compete against each other in Basketball\"@en .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/SportsLeague> .",
			"<http://dbpedia.org/ontology/BasketballLeague> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:BasketballLeague> .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#label> \"Naturereignis\"@de .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#label> \"natural event\"@en .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#label> \"φυσικό γεγονός\"@el .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#label> \"gebeurtenis in de natuur\"@nl .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#label> \"evento naturale\"@it .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#comment> \"Το φυσικό γεγονός χρησιμοποιείται για να περιγράψει ένα συμβάν που πραγματοποιείται φυσικά\"@el .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Event> .",
			"<http://dbpedia.org/ontology/NaturalEvent> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:NaturalEvent> .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"Provinz\"@de .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"province\"@en .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"επαρχία\"@el .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"英語圏の行政区画\"@ja .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"provincie\"@nl .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"province\"@fr .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#label> \"cúige\"@ga .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#comment> \"An administrative body governing a territorial unity on the intermediate level, between local and national level\"@en .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#comment> \"Είναι διοικητική δομή του κράτους που διοικεί μια περιοχή που είναι σε έκταση μεγαλύτερη από τοπικό επίπεδο και μικρότερη από εθνικό επίπεδο.\"@el .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/GovernmentalAdministrativeRegion> .",
			"<http://dbpedia.org/ontology/Province> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:Province> .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"Mondkrater\"@de .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"cratera lunar\"@pt .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"lunar crater\"@en .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"Σεληνιακός κρατήρας\"@el .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"maankrater\"@nl .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"cratère lunaire\"@fr .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#label> \"cráitéar gealaí\"@ga .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Crater> .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q1348589> .",
			"<http://dbpedia.org/ontology/LunarCrater> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:LunarCrater> .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/2000/01/rdf-schema#label> \"motorsport season\"@en .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/2000/01/rdf-schema#label> \"motorsportseizoen\"@nl .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/2000/01/rdf-schema#label> \"Motorsportsaison\"@de .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/SportsSeason> .",
			"<http://dbpedia.org/ontology/MotorsportSeason> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:MotorsportSeason> .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"militärische Person\"@de .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"military person\"@en .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"στρατιωτικός\"@el .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"軍人\"@ja .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"militair\"@nl .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"militare\"@it .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"군인\"@ko .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#label> \"militaire\"@fr .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Person> .",
			"<http://dbpedia.org/ontology/MilitaryPerson> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:MilitaryPerson> .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"Zeitraum\"@de .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"time period\"@en .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"χρονική περίοδος\"@el .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"tijdvak\"@nl .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"période temporelle\"@fr .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"tréimhse\"@ga .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#label> \"periodo temporal\"@es .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://www.w3.org/2002/07/owl#Thing> .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/2002/07/owl#disjointWith> <http://dbpedia.org/ontology/Person> .",
			"<http://dbpedia.org/ontology/TimePeriod> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:TimePeriod> .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"Fahrzeugmotor\"@de .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"motor de automóvel\"@pt .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"automobile engine\"@en .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"κινητήρας αυτοκινήτου\"@el .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"内燃機関\"@ja .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"automotor\"@nl .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"motore d'automobile\"@it .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"자동차 엔진\"@ko .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"moteur d'automobile\"@fr .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#label> \"inneall gluaisteáin\"@ga .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Engine> .",
			"<http://dbpedia.org/ontology/AutomobileEngine> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:AutomobileEngine> .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"Archäologe\"@de .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"archeologist\"@en .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"Αρχαιολόγος\"@el .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"考古学者\"@ja .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"archeoloog\"@nl .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"archéologue\"@fr .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"archeolog\"@pl .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"seandálaí\"@ga .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#label> \"Arqueólogo\"@es .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Person> .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q3621491> .",
			"<http://dbpedia.org/ontology/Archeologist> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:Archeologist> .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"Enzym\"@de .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"enzyme\"@en .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"ένζυμο\"@el .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"酵素\"@ja .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"enzym\"@nl .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"enzima\"@it .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"효소\"@ko .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#label> \"einsím\"@ga .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Biomolecule> .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q8047> .",
			"<http://dbpedia.org/ontology/Enzyme> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:Enzyme> .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#label> \"songwriter\"@en .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#label> \"Liedschreiber\"@de .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#label> \"songwriter (tekstdichter)\"@nl .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#label> \"auteur-compositeur\"@fr .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#comment> \"a person who writes songs.\"@en .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#comment> \"een persoon die de muziek en/of de tekst voor populaire muzieknummers schrijft.\"@nl .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/Writer> .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q753110> .",
			"<http://dbpedia.org/ontology/SongWriter> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:SongWriter> .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"Platz\"@de .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"square\"@en .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"正方形\"@ja .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"plein\"@nl .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"place\"@fr .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#label> \"cearnóg\"@ga .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/ArchitecturalStructure> .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q174782> .",
			"<http://dbpedia.org/ontology/Square> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:Square> .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"Universität\"@de .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"universidade\"@pt .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"university\"@en .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"πανεπιστήμιο\"@el .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"大学\"@ja .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"universiteit\"@nl .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"대학\"@ko .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"université\"@fr .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"uniwersytet\"@pl .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"ollscoil\"@ga .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#label> \"universidad\"@es .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2000/01/rdf-schema#subClassOf> <http://dbpedia.org/ontology/EducationalInstitution> .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2002/07/owl#equivalentClass> <http://schema.org/CollegeOrUniversity> .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/2002/07/owl#equivalentClass> <http://www.wikidata.org/entity/Q3918> .",
			"<http://dbpedia.org/ontology/University> <http://www.w3.org/ns/prov#wasDerivedFrom> <http://mappings.dbpedia.org/index.php/OntologyClass:University> .",
			"<http://dbpedia.org/ontology/AnatomicalStructure> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Class> .",
			"<http://dbpedia.org/ontology/AnatomicalStructure> <http://www.w3.org/2000/01/rdf-schema#label> \"anatomska struktura\"@sl ."
		);

		let mut failed = false;
		let mut prev_wt = wt.clone();
		for string_nr in 0..str_vec.len() {
			let current_str = str_vec.get(string_nr).unwrap();
			match wt.append_str_d(current_str) {
				Ok(_) => {
					println!("string_nr: {} - wt.len(): {}", string_nr, wt.len_d());
					for test_nr in 0..string_nr + 1 {
						let string = str_vec.get(test_nr).unwrap();
						let string_in_wt = wt.access_str_d(test_nr as u64).unwrap();
						if !(string == &string_in_wt) {
							failed = true;
							println!("+++\nlatest str:       [{}], line {}", current_str, string_nr + 1);
							println!("tested str:       [{}], line {}", string, test_nr + 1);
							println!("tested str in wt: [{}]", string_in_wt);
	//						assert_eq!(string, &string_in_wt);
						}
					}
					if failed {
						println!("-----------------\nprev wt: {:?}", prev_wt);
						println!("-----------------\n     wt: {:?}", wt);
						break;
					}
					prev_wt = wt.clone();
				},
				Err(reason) => println!("{}", reason)
			};
		}
	}

	#[test]
	fn doc_example() {
		let sequence1 = DBVec::from_bytes(&[0b00001000]);
		let sequence2 = DBVec::from_bytes(&[0b10000000]);
		let sequence3 = DBVec::from_bytes(&[0b10000100]);
		let sequence4 = DBVec::from_bytes(&[0b11000100, 0b10000000]);

		// Here we create a wavelet trie from these sequences
		let wt = WaveletTrie::from_sequences_d(&[sequence1, sequence2, sequence3, sequence4]);
		println!("{:?}", wt);

		// There should be 4 sequences in the trie now:
		assert_eq!(4, wt.len_d());

		// Let's see at which positions prefix '001' occurs (should be 2 and 3).
		let mut prefix_001 = DBVec::new();
		prefix_001.push(false);
		prefix_001.push(false);
		prefix_001.push(true);
		assert_eq!(vec![2, 3], wt.select_all_d(&prefix_001));
	}
}
