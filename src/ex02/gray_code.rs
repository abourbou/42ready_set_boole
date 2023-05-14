
pub fn gray_code(n : u32) -> u32 {
	n ^ (n >> 1)
}

#[cfg(test)]
	#[test]
	fn test00(){
		assert_eq!(gray_code(0), 0);  // 00
		assert_eq!(gray_code(1), 1);  // 01
		assert_eq!(gray_code(2), 3);  // 11
		assert_eq!(gray_code(3), 2);  // 10
		assert_eq!(gray_code(4), 6);  // 110
		assert_eq!(gray_code(5), 7);  // 111
		assert_eq!(gray_code(6), 5);  // 101
		assert_eq!(gray_code(7), 4);  // 100
		assert_eq!(gray_code(8), 12); // 1100
	}
	#[test]
	fn test01(){
		assert_eq!(gray_code(9), 13); // 1101
		assert_eq!(gray_code(15), 8); // 1000
		assert_eq!(gray_code(37), 55); //110111
	}
	#[test]
	fn test02(){
		assert_eq!(gray_code(78), 105); // 1101001
		assert_eq!(gray_code(396), 330); //101001010
		assert_eq!(gray_code(1582), 1337); //10100111001
	}