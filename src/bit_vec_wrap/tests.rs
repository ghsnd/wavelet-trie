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
		let should_be_prefix = BitVecWrap::from_bytes(&[0b01010101]);
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
		assert_eq!(true, longest_common_prefix.is_empty());
	}

	#[test]
	fn different_suffix() {
		let bv1 = BitVecWrap::from_bytes(&[0b01010101, 0b01010101]);
		let bv2 = BitVecWrap::from_bytes(&[0b01010101, 0b01011101, 0b00011101]);
		let longest_common_prefix = bv1.longest_common_prefix(&bv2);
		let mut should_be_suffix = BitVecWrap::from_bytes(&[0b10100011]);
		should_be_suffix.push(true);
		should_be_suffix.push(false);
		should_be_suffix.push(true);
		let result: (bool, BitVecWrap) = bv2.different_suffix(longest_common_prefix.len());
		assert_eq!(should_be_suffix, result.1);
		assert_eq!(true, result.0);
	}

	#[test]
	fn is_prefix_of() {
		let bv1 = BitVecWrap::from_bytes(&[0b01010101, 0b01011101]);
		let bv2 = BitVecWrap::from_bytes(&[0b01010101, 0b01011101, 0b00011101]);
		let bv3 = BitVecWrap::from_bytes(&[0b01010101, 0b01000001, 0b00011101]);
		assert_eq!(true, bv1.is_prefix_of(&bv2));
		assert_eq!(false, bv1.is_prefix_of(&bv3));
	}

	#[test]
	fn select() {
		let bv = BitVecWrap::from_bytes(&[0b01010101, 0b01010101]);
		assert_eq!(0, bv.select(false, 1));
		assert_eq!(2, bv.select(false, 2));
		assert_eq!(4, bv.select(false, 3));
		assert_eq!(6, bv.select(false, 4));
		assert_eq!(8, bv.select(false, 5));
		assert_eq!(10, bv.select(false, 6));
		assert_eq!(12, bv.select(false, 7));
		assert_eq!(14, bv.select(false, 8));
		assert_eq!(1, bv.select(true, 1));
		assert_eq!(3, bv.select(true, 2));
		assert_eq!(5, bv.select(true, 3));
		assert_eq!(7, bv.select(true, 4));
		assert_eq!(9, bv.select(true, 5));
		assert_eq!(11, bv.select(true, 6));
		assert_eq!(13, bv.select(true, 7));
		assert_eq!(15, bv.select(true, 8));
	}
}
