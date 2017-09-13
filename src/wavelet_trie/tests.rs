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
	}
}
