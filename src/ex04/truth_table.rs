
use std::collections::BTreeMap;
use std::io::{Write, stdout};

use crate::ex03::boolean_evaluation::eval_formula;

const ALLOWED_CHAR : &str = "!&|^>=";

pub fn print_truth_table(formula: &str) {

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

	for (key, _value) in map_var.iter() {
		print!("| {} ", key);
		stdout().flush().unwrap();
	}
	println!("| = |");

	let mut buffer_formula : Vec<char> = formula.chars().collect();
	for nb in 0..(1 << map_var.len()) {
		let mut i = map_var.len();
		for vec_pos in map_var.values() {
			i -= 1;
			let val = char::from_digit(nb >> i & 1, 2).unwrap();
			for pos in vec_pos.iter() {
				buffer_formula[*pos] = val;
			}
			print!("| {} ", val);
			std::io::stdout().flush().unwrap();
		}

		let current_formula : String = buffer_formula.iter().collect();
		println!("| {} | from {}", eval_formula(current_formula.as_str()) as u32, current_formula);
	}
}

#[cfg(test)]
	#[test]
	#[should_panic]
	fn test_double_char() {
		print_truth_table("AVSVDDSV");
	}
	#[test]
	#[should_panic]
	fn test_inval_char() {
		print_truth_table("AV2");
	}

	#[test]
	fn test_valid00() {
		print_truth_table("AB&C|");
	}
	#[test]
	fn test_valid01() {
		print_truth_table("AB>");
	}
	#[test]
	fn test_valid02() {
		print_truth_table("AB>C|");
	}
	#[test]
	fn test_valid03() {
		print_truth_table("AB=");
	}
