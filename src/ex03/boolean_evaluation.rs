

use id_tree::*;
use id_tree::InsertBehavior::*;

const SOLO_OPERATOR_LIST : &str = "!";
const DUO_OPERATOR_LIST : &str = "&|^>=";

pub fn eval_formula(formula: &str) -> bool {
	let tree = create_btree(formula);
	evaluate_btree(&tree)
}

pub fn create_btree(formula: &str) -> Tree<char> {

	if formula.is_empty() {
		panic!("Formula must not be empty");
	}
	// Create tree
	let mut curr_formula = String::from(formula);
	let mut tree = TreeBuilder::<char>::new()
				.with_node_capacity(formula.len())
				.build();
	// Start recursion
	let last_char = curr_formula.pop().unwrap();
	if last_char == '0' || last_char == '1' {
		tree.insert(Node::new(last_char), AsRoot).unwrap();
	} else if SOLO_OPERATOR_LIST.contains(last_char) {
		create_operator_node(&mut tree, AsRoot, &mut curr_formula, last_char, 1);
	} else if DUO_OPERATOR_LIST.contains(last_char) {
		create_operator_node(&mut tree, AsRoot, &mut curr_formula, last_char, 2);
	} else {
		panic!("Invalid character");
	}

	if !curr_formula.is_empty() {
		panic!("Invalid formula : too much characters");
	}

	tree
}

fn create_operator_node(tree: &mut Tree<char>, prev_node_id : InsertBehavior, curr_formula : &mut String, operator : char, nbr_arg : usize)
{
	let node_id = tree.insert(Node::new(operator), prev_node_id).unwrap();
	for _i in 0..nbr_arg {
		let last_char = curr_formula.pop();
		match last_char {
			None => panic!("invalid formula : incomplete"),
			Some(c) => {
				if c == '0' || c == '1' {
					tree.insert(Node::new(c), UnderNode(&node_id)).unwrap();
				} else if SOLO_OPERATOR_LIST.contains(c) {
					create_operator_node(tree, UnderNode(&node_id), curr_formula, c, 1);
				} else if DUO_OPERATOR_LIST.contains(c) {
					create_operator_node(tree, UnderNode(&node_id), curr_formula, c, 2);
				} else {
					panic!("invalid character");
				}
			}
		}
	}
}

fn eval_node(tree : &Tree<char>, node_id: &NodeId) -> bool {

	let curr_node = tree.get(node_id).unwrap();

	match curr_node.data() {
		'0' => false,
		'1' => true,
		'!' => !eval_node(tree, &curr_node.children()[0]),
		'&' =>  eval_node(tree, &curr_node.children()[0]) && eval_node(tree, &curr_node.children()[1]),
		'|' =>  eval_node(tree, &curr_node.children()[0]) || eval_node(tree, &curr_node.children()[1]),
		'^' =>  eval_node(tree, &curr_node.children()[0]) ^  eval_node(tree, &curr_node.children()[1]),
		'>' =>  eval_node(tree, &curr_node.children()[0]) || !eval_node(tree, &curr_node.children()[1]),
		'=' =>  eval_node(tree, &curr_node.children()[0]) == eval_node(tree, &curr_node.children()[1]),
		_ => panic!("Invalid caracter"),
	}
}

// evaluate the tree starting from the root node
fn evaluate_btree(tree : &Tree<char>) -> bool {
	eval_node(tree, tree.root_node_id().unwrap())
}


#[cfg(test)]
	#[test]
	#[should_panic]
	fn test_empty() {
		eval_formula("");
	}
	#[test]
	#[should_panic]
	fn test_invalid_char() {
		eval_formula("123456789");
	}
	#[test]
	#[should_panic]
	fn test_too_long_formula() {
		eval_formula("111&");
	}
	#[test]
	#[should_panic]
	fn test_too_short_formula() {
		eval_formula("1&");
	}

	// Btree creation
	#[test]
	fn test_create_btree00() {
		let tree = create_btree("1");
		let mut s = String::new();
		tree.write_formatted(&mut s).unwrap();
		assert_eq!(&s, "'1'\n");
	}
	#[test]
	fn test_create_btree01() {
		let tree = create_btree("01|");
		let mut s = String::new();
		tree.write_formatted(&mut s).unwrap();
		assert_eq!(&s, "\
'|'
├── '1'
└── '0'
");
	}
	#[test]
	fn test_create_btree02() {
		let tree = create_btree("01&0|");
		let mut s = String::new();
		tree.write_formatted(&mut s).unwrap();
		assert_eq!(&s, "\
'|'
├── '0'
└── '&'
    ├── '1'
    └── '0'
");
	}
	#[test]
	fn test_create_btree04() {
		let tree = create_btree("1!");
		let mut s = String::new();
		tree.write_formatted(&mut s).unwrap();
		assert_eq!(&s, "\
'!'
└── '1'
");
	}

	// Evaluation result
	#[test]
	fn test_eval00() {
		assert_eq!(eval_formula("1"), true);
	}
	#[test]
	fn test_eval01() {
		assert_eq!(eval_formula("0"), false);
	}
	#[test]
	fn test_eval02() {
		assert_eq!(eval_formula("0!"), true);
	}
	#[test]
	fn test_eval03() {
		assert_eq!(eval_formula("10&"), false);
	}
	#[test]
	fn test_eval04() {
		assert_eq!(eval_formula("10|"), true);
	}
	#[test]
	fn test_eval05() {
		assert_eq!(eval_formula("11>"), true);
	}
	#[test]
	fn test_eval06() {
		assert_eq!(eval_formula("11^"), false);
	}
	#[test]
	fn test_eval07() {
		assert_eq!(eval_formula("10="), false);
	}
	#[test]
	fn test_eval08() {
		assert_eq!(eval_formula("1011||="), true);
	}
	#[test]
	fn test_eval09() {
		assert_eq!(eval_formula("00|1&!"), true);
	}
	#[test]
	fn test_eval10() {
		assert_eq!(eval_formula("10|1&!"), false);
	}
	#[test]
	fn test_eval11() {
		assert_eq!(eval_formula("11|0&!"), true);
	}
	#[test]
	fn test_eval12() {
		assert_eq!(eval_formula("10>"), false);
	}
