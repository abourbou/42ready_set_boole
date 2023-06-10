
use std::mem;

// https://en.wikipedia.org/wiki/Hilbert_curve#Applications_and_mapping_algorithms

pub fn map(x: u16, y: u16) -> f64 {
	let n = u16::MAX as u32 + 1;
	if x > (n - 1) as u16 || y > (n - 1) as u16{
		panic!("x and y must be in [0,{}]", n - 1);
	}
	let mut x = x;
	let mut y = y;
	let (mut rx, mut ry, mut dist, mut bit) : (u16, u16, f64, u16);
	(dist, bit) = (0.,  (n / 2) as u16);
	while bit > 0 {
		rx = ((x & bit) > 0) as u16;
		ry = ((y & bit) > 0) as u16;
		dist += bit as f64 * bit as f64 * ((3 * rx) ^ ry) as f64;
		rot(n, &mut x, &mut y, rx != 0, ry != 0);
		bit >>= 1;
	}

	dist / (u32::MAX as f64)
}

pub fn rot(n : u32, x: &mut u16, y: &mut u16, rx: bool, ry: bool) {

	if n > (u16::MAX as u32 + 1) {
		panic!("n must be in the range [0, 2**16]");
	}
	if !ry{
		if rx {
			*x = (n - 1) as u16 - *x;
			*y = (n - 1) as u16 - *y;
		}
		mem::swap(x, y);
	}
}


#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;
	use rand::distributions::{Distribution, Uniform};
	use ordered_float::OrderedFloat;
	use super::*;

	#[test]
	fn basic() {
		assert_eq!(0., map(0, 0));

		assert_eq!(1., map(u16::MAX, 0));

		assert_eq!(1. / 3., map(0, u16::MAX));

		assert_eq!(2. / 3., map(u16::MAX, u16::MAX));
	}
	#[test]
	fn random() {
		let range = Uniform::from(0..u16::MAX as u32 + 1);
		let mut rng = rand::thread_rng();
		let mut set_result = BTreeMap::<OrderedFloat::<f64>, (u16, u16)>::new();
		let limit : u64 = 500000;
		for _i in 0 as u64..limit {
			let x = range.sample(&mut rng) as u16;
			let y = range.sample(&mut rng) as u16;
			let result = map(x,y);
			let old_value = set_result.insert(OrderedFloat(result), (x,y));
			if old_value.is_some() {
				let old_entry = old_value.unwrap();
				if old_entry == (x,y) {
					continue;
				}
				else {
					panic!("duplicated entry for [{}. {}]
					and [{}, {}] at {}", old_entry.0, old_entry.1, x, y, result);
				}
			}
		}
	}
}
