
use id_tree::*;
use id_tree::InsertBehavior::*;
use id_tree::RemoveBehavior::*;
use id_tree::SwapBehavior::*;

use crate::ex05::negation_normal_form::{negation_normal_form, create_btree,
										copy_subtree, insert_subtree};

// TODO : put at negation form
// TODO : put every operator to the right (aka the last position)
// ? Is it necessary
// TODO : extend operator
// TODO 1 : AB&C| {(A&B) | C} => AC|BC|&
// TODO 2 : ABC&| {(A&B) | C} => ACAB||&
// TODO 3 : AB&C| {(A&B) | C} => ACAB||&   {(A|C) & (A|B)}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let formula = negation_normal_form(formula);

    let mut tree = create_btree(&formula);
	while let Some((or_node_id, pose_wrong)) = find_disjunction(&tree) {
		disjunction_to_conjunction(&mut tree, or_node_id, pose_wrong);
	}

	//? Put op first each time
	// while let Some(op_id) = find_wrong_place_op(&tree) {
	// 	tree.make_first_sibling(&op_id).unwrap();
	// }

	tree_to_string(&tree)
}

// Find if a & is after a |
fn find_disjunction(tree: &Tree<char>) -> Option<(NodeId, i32)> {

	let it_node = tree.traverse_level_order_ids(tree.root_node_id().unwrap()).unwrap();
	for node_id in it_node {
		if tree.get(&node_id).unwrap().data() == &'|' {
			let children_id1 = tree.get(&node_id).unwrap().children().iter().next().unwrap();
			let mut pose_wrong = 0;
			if tree.get(children_id1).unwrap().data() == &'&'{
				pose_wrong += 1;
			}
			let children_id2 = tree.get(&node_id).unwrap().children().iter().nth(1).unwrap();
			if tree.get(children_id2).unwrap().data() == &'&'{
				pose_wrong += 2;
			}
			if pose_wrong > 0 {
				return Some((node_id, pose_wrong));
			}
		}
	}
	None
}

fn disjunction_to_conjunction(tree : &mut Tree<char>, or_node_id : NodeId, pose_wrong : i32) {
	match pose_wrong {
		// case ABC&| => AB|AC|&
		1 => {
			// Create subtree for A,B,C
			let mut it = tree.traverse_level_order_ids(&or_node_id).unwrap();
			let subtree_a = copy_subtree(tree, &it.nth(2).unwrap());
			let subtree_c = copy_subtree(tree, &it.next().unwrap());
			let subtree_b = copy_subtree(tree, &it.next().unwrap());

			// Transform to AB|AC|&
			let and_id = tree.insert(Node::new('&'), UnderNode(&or_node_id)).unwrap();
			tree.swap_nodes(&and_id, &or_node_id, TakeChildren).unwrap();
			tree.remove_node(or_node_id, DropChildren).unwrap();
			let left_id = tree.insert(Node::new('|'), UnderNode(&and_id)).unwrap();
			let right_id = tree.insert(Node::new('|'), UnderNode(&and_id)).unwrap();
			insert_subtree(tree, &subtree_c, &left_id);
			insert_subtree(tree, &subtree_a, &left_id);
			insert_subtree(tree, &subtree_b, &right_id);
			insert_subtree(tree, &subtree_a, &right_id);
		}
		// case AB&C| => AC|BC|&
		2 => {
			// Create subtree for A,B,C
			let mut it = tree.traverse_level_order_ids(&or_node_id).unwrap();
			let subtree_c = copy_subtree(tree, &it.nth(1).unwrap());
			let subtree_b = copy_subtree(tree, &it.nth(1).unwrap());
			let subtree_a = copy_subtree(tree, &it.next().unwrap());
			// Transform to AC|BC|&
			let and_id = tree.insert(Node::new('&'), UnderNode(&or_node_id)).unwrap();
			tree.swap_nodes(&and_id, &or_node_id, TakeChildren).unwrap();
			tree.remove_node(or_node_id, DropChildren).unwrap();
			let left_id = tree.insert(Node::new('|'), UnderNode(&and_id)).unwrap();
			let right_id = tree.insert(Node::new('|'), UnderNode(&and_id)).unwrap();
			insert_subtree(tree, &subtree_c, &left_id);
			insert_subtree(tree, &subtree_b, &left_id);
			insert_subtree(tree, &subtree_c, &right_id);
			insert_subtree(tree, &subtree_a, &right_id);
		}
		// case AB&CD&| => AC|AD|BC|BD|&&&
		3 => {
			// Create subtree for A,B,C,D
			let mut it = tree.traverse_level_order_ids(&or_node_id).unwrap();
			let subtree_d = copy_subtree(tree, &it.nth(3).unwrap());
			let subtree_c = copy_subtree(tree, &it.next().unwrap());
			let subtree_b = copy_subtree(tree, &it.next().unwrap());
			let subtree_a = copy_subtree(tree, &it.next().unwrap());
			// Transform to AC|AD|BC|BD|&&&
			let and_id0 = tree.insert(Node::new('&'), UnderNode(&or_node_id)).unwrap();
			tree.swap_nodes(&and_id0, &or_node_id, TakeChildren).unwrap();
			tree.remove_node(or_node_id, DropChildren).unwrap();
			let and_id1 = tree.insert(Node::new('&'), UnderNode(&and_id0)).unwrap();
			let and_id2 = tree.insert(Node::new('&'), UnderNode(&and_id1)).unwrap();

			let or_id0 = tree.insert(Node::new('|'), UnderNode(&and_id0)).unwrap();
			let or_id1 = tree.insert(Node::new('|'), UnderNode(&and_id1)).unwrap();
			let or_id2 = tree.insert(Node::new('|'), UnderNode(&and_id2)).unwrap();
			let or_id3 = tree.insert(Node::new('|'), UnderNode(&and_id2)).unwrap();
			insert_subtree(tree, &subtree_c, &or_id0);
			insert_subtree(tree, &subtree_a, &or_id0);
			insert_subtree(tree, &subtree_d, &or_id1);
			insert_subtree(tree, &subtree_a, &or_id1);
			insert_subtree(tree, &subtree_d, &or_id2);
			insert_subtree(tree, &subtree_b, &or_id2);
			insert_subtree(tree, &subtree_c, &or_id3);
			insert_subtree(tree, &subtree_b, &or_id3);
		}
		_ => panic!("Invalid pose for & operators")
	}
}

#[allow(dead_code)]
fn find_wrong_place_op(tree: &Tree<char>) -> Option<NodeId>{
	for node in tree.traverse_level_order(tree.root_node_id().unwrap()).unwrap() {
		if node.data() == &'|' || node.data() == &'&' {
			let child1 = tree.get(&node.children()[0]).unwrap();
			let child2 = tree.get(&node.children()[1]).unwrap();
			let priority_1 = match child1.data() {
				'&' => 2,
				'|' => 1,
				_ => 0,
			};
			let priority_2 = match child2.data() {
				'&' => 2,
				'|' => 1,
				_ => 0,
			};
			if priority_2 > priority_1 {
				return Some(node.children()[1].clone());
			}
		}
	}
	None
}

fn	tree_to_string(tree: &Tree<char>) -> String {
	let mut result_str = String::new();
	for node in tree.traverse_pre_order(tree.root_node_id().unwrap()).unwrap() {
		result_str.push(*node.data());
	}

	result_str.chars().rev().collect()
}

#[cfg(test)]
    // * println!("{}", conjunctive_normal_form("AB&!"));
    // * // A!B!|
    // * println!("{}", conjunctive_normal_form("AB|!"));
    // * // A!B!&
    // * println!("{}", conjunctive_normal_form("AB|C&"));
    // * // AB|C&
    // * println!("{}", conjunctive_normal_form("AB|C|D|"));
    // * // ABCD|||
    // * println!("{}", conjunctive_normal_form("AB&C&D&"));
    // * // ABCD&&&
    // * println!("{}", conjunctive_normal_form("AB&!C!|"));
    // * // A!B!C!||
    // * println!("{}", conjunctive_normal_form("AB|!C!&"));
    // * // A!B!C!&&

    #[test]
    fn test_00() {
        let tree_str = conjunctive_normal_form("AB&!");
		assert_eq!(tree_str, "A!B!|");
	}
	#[test]
	fn test_01() {
		let tree_str = conjunctive_normal_form("AB|!");
		assert_eq!(tree_str, "A!B!&");
	}

	#[test]
	fn test_02() {
		let tree_str = conjunctive_normal_form("AB|C&");
		assert_eq!(tree_str, "AB|C&");
	}
	#[test]
	fn test_03() {
		let tree_str = conjunctive_normal_form("AB|C|D|");
		assert_eq!(tree_str, "AB|C|D|");
	}
	#[test]
	fn test_04() {
		let tree_str = conjunctive_normal_form("AB&C&D&");
		assert_eq!(tree_str, "AB&C&D&");
	}
	#[test]
	fn test_05() {
		let tree_str = conjunctive_normal_form("AB&!C!|");
		assert_eq!(tree_str, "A!B!|C!|");
	}
	#[test]
	fn test_06() {
		let tree_str = conjunctive_normal_form("AB|!C!&");
		assert_eq!(tree_str, "A!B!&C!&");
	}
	#[test]
	fn test_07() {
		let tree_str = conjunctive_normal_form("AB&C|");
		assert_eq!(tree_str, "AC|BC|&");
	}
	#[test]
	fn test_08() {
		let tree_str = conjunctive_normal_form("AB&CD&|");
		assert_eq!(tree_str, "AC|AD|BC|BD|&&&");
	}
