use std::collections::BTreeMap;

use crate::ex03::boolean_evaluation::eval_formula;

const ALLOWED_CHAR : &str = "!&|^>=";


pub fn sat(formula: &str) -> bool {

	let mut map_var = BTreeMap::<char, Vec<usize>>::new();

	if formula.is_empty() {
		panic!("Empty formula");
	}
	for i in 0..formula.len() {
		let c = formula.chars().nth(i).unwrap();
		if ALLOWED_CHAR.contains(c) {}
		else if c.is_ascii_uppercase() {
			match map_var.get_mut(&c) {
				Some(vec) => vec.push(i),
				None => {map_var.insert(c, vec![i]);},
			};
		} else {
			panic!("Unvalid formula : unknown character");
		}
	}

	let mut buffer_formula : Vec<char> = formula.chars().collect();
	for nb in 0..(1 << map_var.len()) {
		let mut i = map_var.len();
		for vec_pos in map_var.values() {
			i -= 1;
			let val = char::from_digit(nb >> i & 1, 2).unwrap();
			for pos in vec_pos.iter() {
				buffer_formula[*pos] = val;
			}
		}
		let current_formula : String = buffer_formula.iter().collect();
		if eval_formula(current_formula.as_str()) {
			return true;
		}
	}

	false
}

#[cfg(test)]
	#[test]
	fn test_00() {
		assert_eq!(sat("AB|"), true);
	}
	#[test]
	fn test_01() {
		assert_eq!(sat("AB&"), true);
	}
	#[test]
	fn test_02() {
		assert_eq!(sat("AA!&"), false);
	}
	#[test]
	fn test_03() {
		assert_eq!(sat("AA^"), false);
	}
