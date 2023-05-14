
use crate::ex00::adder::adder;

pub fn multiplier(mut a : u32, mut b : u32) -> u32 {
	let mut result : u32 = 0;

	while b > 0 {
		if b & 1 == 1 {
			result = adder(result, a);
		}
		a = a << 1;
		b = b >> 1;
	}

	result
}

#[cfg(test)]
use std::num::Wrapping;
	#[test]
	fn test00() {
		let a = 0;
		let b = 4;
		assert_eq!(multiplier(a,b), 0);
	}
	#[test]
	fn test01() {
		let a = 1;
		let b = 4;
		assert_eq!(multiplier(a,b), 4);
	}
	#[test]
	fn test02() {
		let a = 4;
		let b = 1;
		assert_eq!(multiplier(a,b), 4);
	}
	#[test]
	fn test03() {
		let a = 7;
		let b = 6;
		assert_eq!(multiplier(a,b), 42);
	}
	#[test]
	fn test04() {
		let a = 45105;
		let b = 9854;
		assert_eq!(multiplier(a,b), 444464670);
	}
	#[test]
	fn test_overflow00() {
		let a = std::u32::MAX;
		let b = 6;
		let w_a = Wrapping(a);
		let w_b = Wrapping(b);
		assert_eq!(multiplier(a,b), (w_a * w_b).0);
	}
	#[test]
	fn test_overflow01() {
		let a = std::u32::MAX;
		let b = std::u32::MAX;
		let w_a = Wrapping(a);
		let w_b = Wrapping(b);
		assert_eq!(multiplier(a,b), (w_a * w_b).0);
	}
