
use crate::ex10::curve::rot;

// https://en.wikipedia.org/wiki/Hilbert_curve#Applications_and_mapping_algorithms

pub fn inverse_map(n: f64) -> (u16, u16) {
	if !(0. ..= 1.).contains(&n) {
		panic!("map only in [0, 1]");
	}
	let dist = (n * u32::MAX as f64) as u32;
	let(mut x, mut y, mut rx, mut ry, mut bit) : (u16, u16, u16, u16, u32);
	(x, y) = (0, 0);
	let mut t = dist;
	bit = 1;
	while bit < (u16::MAX as u32 + 1) {
		rx = (1 & (t / 2)) as u16;
		ry = (1 & (t ^ rx as u32)) as u16;
		rot(bit, &mut x, &mut y, rx != 0, ry != 0);
		x += bit as u16 * rx;
		y += bit as u16 * ry;

		bit *= 2;
		t /= 4;
	}

	(x, y)
}

#[cfg(test)]
mod tests {

	use rand::distributions::{Distribution, Uniform};
	use crate::ex10::curve::map;
	use super::*;

	#[test]
	fn random() {
		let range = Uniform::from(0..u16::MAX as u32 + 1);
		let mut rng = rand::thread_rng();
		let limit : u64 = 500000;
		for _i in 0 as u64..limit {
			let x = range.sample(&mut rng) as u16;
			let y = range.sample(&mut rng) as u16;
			let result = map(x,y);
			let inverse_result = inverse_map(result);
			if inverse_result != (x,y) {
				panic!("For [{}, {}], inv is [{}, {}]", x, y, inverse_result.0, inverse_result.1);
			}
		}
	}
}
