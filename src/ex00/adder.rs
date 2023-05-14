
pub fn adder(mut a : u32, mut b : u32) -> u32 {
	let mut carry : u32;

	while b != 0 {
		carry = a & b;
		a = a ^ b;
		b = carry << 1;
	}

	a
}

#[cfg(test)]
	#[test]
	fn test00() {
		let a : u32 = 0;
		let b : u32 = 0;
		assert_eq!(adder(a,b), 0);
	}
	#[test]
	fn test01() {
		let a : u32 = 0;
		let b : u32 = 4;
		assert_eq!(adder(a,b), 4);
	}
	#[test]
	fn test02() {
		let a : u32 = 3;
		let b : u32 = 2;
		assert_eq!(adder(a,b), 5);
	}
	#[test]
	fn test_overflow00() {
		let a : u32 = std::u32::MAX;
		let b : u32 = 8;
		assert_eq!(adder(a,b), 7);
	}
	#[test]
	fn test_overflow01() {
		let a : u32 = std::u32::MAX;
		let b : u32 = std::u32::MAX;
		assert_eq!(adder(a,b), std::u32::MAX - 1);
	}
