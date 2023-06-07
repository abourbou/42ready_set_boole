
use std::collections::{BTreeSet, BTreeMap};
use id_tree::*;

use crate::ex05::negation_normal_form::create_btree;

const ALLOWED_CHAR : &str = "!&|^>=";

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {

	let mut map_set = BTreeMap::<char, BTreeSet::<i32>>::new();

	if formula.is_empty() {
		panic!("Empty formula");
	}
	for i in 0..formula.len() {
		let c = formula.chars().nth(i).unwrap();
		if ALLOWED_CHAR.contains(c) {}
		else if c.is_ascii_uppercase() {
			if map_set.get(&c).is_none() {
				let set = sets.get(map_set.len());
				if set.is_none() {
					panic!("Number of variables doesn't corresponds to the number of sets");
				}
				map_set.insert(c, set.unwrap().iter().cloned().collect());
			}
		} else {
			panic!("Unvalid formula : unknown character");
		}
	}

	if map_set.len() != sets.len() {
		panic!("Number of variables doesn't corresponds to the number of sets");
	}

	let mut global_set : BTreeSet::<i32> = BTreeSet::new();
	for set in map_set.values() {
		global_set = &global_set | set;
	}

	println!("all the sets  : {:?}", map_set);
	println!("global set : {:?}", global_set);

	let final_set = eval_formula(formula, &map_set, &global_set);
	let mut result = Vec::<i32>::new();
	for val in final_set.iter() {
		result.push(*val);
	}

	result
}

fn eval_formula(formula : &str, map_set : &BTreeMap<char, BTreeSet<i32>>, global_set : &BTreeSet<i32>) -> BTreeSet<i32> {
	let tree = create_btree(formula);
	match tree.root_node_id() {
		Some(root_id) => eval_op(&tree, root_id, map_set, global_set),
		None => BTreeSet::<i32>::new(),
	}
}

fn eval_op(tree : &Tree<char>, node_id : &NodeId, map_set : &BTreeMap<char, BTreeSet<i32>>, global_set : &BTreeSet<i32>) -> BTreeSet<i32> {
	let node = tree.get(node_id).unwrap();
	let content = *node.data();

	if content.is_ascii_uppercase() {
		return map_set.get(&content).unwrap().clone();
	}
	else if content == '&' {
		return	&eval_op(tree, &node.children()[0], map_set, global_set)
				& &eval_op(tree, &node.children()[1], map_set, global_set);
	}
	else if content == '|' {
		return	&eval_op(tree, &node.children()[0], map_set, global_set)
				| &eval_op(tree, &node.children()[1], map_set, global_set);
	}
	else if content == '!' {
		return	&eval_op(tree, &node.children()[0], map_set, global_set)
				^ global_set;
	}
	else if content == '^' {
		return	&eval_op(tree, &node.children()[0], map_set, global_set)
				^ &eval_op(tree, &node.children()[1], map_set, global_set);
	}
	else if content == '>' { // AB> = !A | B = (A ^ global) | B
		return	&(&eval_op(tree, &node.children()[1], map_set, global_set) ^ global_set)
				| &eval_op(tree, &node.children()[0], map_set, global_set);
	}
	else if content == '=' {
		let btree_a = eval_op(tree, &node.children()[1], map_set, global_set);
		let btree_b = eval_op(tree, &node.children()[0], map_set, global_set);
		if (&btree_a ^ &btree_b).is_empty() {
			return btree_a;
		}
		else {
			return BTreeSet::<i32>::new();
		}
	}
	else {
		panic!("Unvalid caracter");
	}
}

#[cfg(test)]
	#[test]
	#[should_panic]
	fn test_invalid_00() {
		eval_set("AB&", vec![]);
	}
	#[test]
	#[should_panic]
	fn test_invalid_01() {
		eval_set("AB&", vec![vec![1], vec![2,30], vec![2]]);
	}
	#[test]
	fn test_00() {
		assert_eq!(eval_set("AB&", vec![vec![1,2,3], vec![1,2,4]]), vec![1,2]);
	}
	#[test]
	fn test_01() {
		assert_eq!(eval_set("AB|", vec![vec![1,2,3], vec![1,2,4]]), vec![1,2,3,4]);
	}
	#[test]
	fn test_02() {
		assert_eq!(eval_set("AB>", vec![vec![1,2,3], vec![1,2,4]]), vec![1,2,4]);
	}
	#[test]
	fn test_03() {
		assert_eq!(eval_set("AB=", vec![vec![1,2,3], vec![1,2,4]]), vec![]);
	}
	#[test]
	fn test_04() {
		assert_eq!(eval_set("AB=", vec![vec![1,2,3], vec![1,2,3]]), vec![1,2,3]);
	}
	#[test]
	fn test_05() {
		assert_eq!(eval_set("A!B&", vec![vec![1,2,3], vec![1,2,4]]), vec![4]);
	}
	#[test]
	fn test_06() {
		assert_eq!(eval_set("AB&", vec![vec![0,1,2], vec![0,3,4]]), vec![0]);
	}
	#[test]
	fn test_07() {
		assert_eq!(eval_set("AB|", vec![vec![0,1,2], vec![3,4,5]]), vec![0,1,2,3,4,5]);
	}
	#[test]
	fn test_08() {
		assert_eq!(eval_set("A!", vec![vec![0,1,2]]), vec![]);
	}
