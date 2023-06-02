
use id_tree::*;
use id_tree::InsertBehavior::*;
use id_tree::RemoveBehavior::*;
use id_tree::SwapBehavior::*;

const SOLO_OPERATOR_LIST : &str = "!";
const DUO_OPERATOR_LIST : &str = "&|^>=";

//* AB|AB&^
//* AB|AB&!&AB|!AB&&|
//* A!! => A
//* AB&! => A!B!|
//* AB|! => A!B!&
//* AB^ => AB!&A!B&|
//* AB> => A!B|
//* AB= => AB&A!B!&|

pub fn negation_normal_form(formula: &str) -> String {

	let mut tree = create_btree(formula);

	// Replace forbidden operators
	while let Some((forbidd_id, forbidd_char)) = find_forbidden_op(&tree) {
		erase_forbidd_op(&mut tree, forbidd_id, forbidd_char);
	}

	// Replace negation operators
	while let Some((neg_id, forbidd_char)) =  find_invalid_neg(&tree) {
		erase_neg(&mut tree, neg_id, forbidd_char);
	}

	let mut result_str = String::new();
	for node in tree.traverse_pre_order(tree.root_node_id().unwrap()).unwrap() {
		result_str.push(*node.data());
	}

	result_str.chars().rev().collect()
}

fn erase_forbidd_op(tree: &mut Tree<char>, forbidd_id : NodeId, forbidd_char : char) {
	let b_id = tree.traverse_level_order_ids(&forbidd_id).unwrap().nth(1).unwrap();
	let a_id = tree.traverse_level_order_ids(&forbidd_id).unwrap().nth(2).unwrap();
	let a_subtree = copy_subtree(tree, &a_id);
	let b_subtree = copy_subtree(tree, &b_id);
	match forbidd_char {
		'^' => {
			// Change AB^ to &&| and then add A and B
			tree.remove_node(a_id, DropChildren).unwrap();
			tree.remove_node(b_id, DropChildren).unwrap();
			*tree.get_mut(&forbidd_id).unwrap().data_mut() = '|';
			let left_id = tree.insert(Node::new('&'), UnderNode(&forbidd_id)).unwrap();
			let right_id = tree.insert(Node::new('&'), UnderNode(&forbidd_id)).unwrap();
			// left : AB!&
			let neg_id = tree.insert(Node::new('!'), UnderNode(&left_id)).unwrap();
			insert_subtree(tree, & b_subtree, &neg_id);
			insert_subtree(tree, &a_subtree, &left_id);
			// right : A!B&
			insert_subtree(tree, & b_subtree, &right_id);
			let neg_id = tree.insert(Node::new('!'), UnderNode(&right_id)).unwrap();
			insert_subtree(tree, &a_subtree, &neg_id);
		}
		'=' => {
			// Change AB^ to &&| and then add A and B
			tree.remove_node(a_id, DropChildren).unwrap();
			tree.remove_node(b_id, DropChildren).unwrap();
			*tree.get_mut(&forbidd_id).unwrap().data_mut() = '|';
			let left_id = tree.insert(Node::new('&'), UnderNode(&forbidd_id)).unwrap();
			let right_id = tree.insert(Node::new('&'), UnderNode(&forbidd_id)).unwrap();
			// left : AB&
			insert_subtree(tree, & b_subtree, &left_id);
			insert_subtree(tree, &a_subtree, &left_id);
			// right : A!B!&
			let neg_id_b = tree.insert(Node::new('!'), UnderNode(&right_id)).unwrap();
			let neg_id_a = tree.insert(Node::new('!'), UnderNode(&right_id)).unwrap();
			insert_subtree(tree, & b_subtree, &neg_id_b);
			insert_subtree(tree, &a_subtree, &neg_id_a);
		}
		'>' => {
			*tree.get_mut(&forbidd_id).unwrap().data_mut() = '|';
			insert_neg(tree, &a_id);
			print_tree(tree);
		}
		_ => panic!("Invalid forbidden character in erase_forbidd_op")
	}
}

pub fn erase_neg(tree: &mut Tree<char>, neg_id : NodeId, forbidd_char : char) {

	let children_id = tree.traverse_level_order_ids(&neg_id).unwrap().nth(2).unwrap();
	let operat_id = tree.get(&neg_id).unwrap().children().iter().next().unwrap().clone();

	match forbidd_char {
		'!' => {
			tree.swap_nodes(&neg_id, &children_id, TakeChildren).unwrap();
			tree.remove_node(neg_id, DropChildren).unwrap();
		}
		'&' => {
		// Change AB&! to A!B!|!
		*tree.get_mut(&operat_id).unwrap().data_mut() = '|';
		insert_neg(tree, &tree.children_ids(&operat_id).unwrap().next().unwrap().clone());
		insert_neg(tree, &tree.children_ids(&operat_id).unwrap().nth(1).unwrap().clone());
		// Remove neg
		tree.swap_nodes(&operat_id, &neg_id, TakeChildren).unwrap();
		tree.remove_node(neg_id, DropChildren).unwrap();
		}
		'|' => {
		// Change AB|! to A!B!&!
		*tree.get_mut(&operat_id).unwrap().data_mut() = '&';
		insert_neg(tree, &tree.children_ids(&operat_id).unwrap().next().unwrap().clone());
		insert_neg(tree, &tree.children_ids(&operat_id).unwrap().nth(1).unwrap().clone());
		// Remove neg
		tree.swap_nodes(&operat_id, &neg_id, TakeChildren).unwrap();
		tree.remove_node(neg_id, DropChildren).unwrap();
		}
		_ => panic!("Forbidden char")
	}
}

fn find_forbidden_op(tree: &Tree<char>) -> Option<(NodeId, char)> {

	let it_node = tree.traverse_level_order_ids(tree.root_node_id().unwrap()).unwrap();
	for node_id in it_node {
		let c = tree.get(&node_id).unwrap().data();
		if c == &'^' || c == &'>' || c == &'=' {
			return Some((node_id, *c));
		}
	}
	None
}

fn find_invalid_neg(tree: &Tree<char>) -> Option<(NodeId, char)> {

	let it_node = tree.traverse_level_order_ids(tree.root_node_id().unwrap()).unwrap();
	for node_id in it_node {
		if tree.get(&node_id).unwrap().data() == &'!' {
			let children_id = tree.get(&node_id).unwrap().children().iter().next().unwrap();
			let c = tree.get(children_id).unwrap().data();
			if !c.is_ascii_alphabetic(){
				return Some((node_id, *c));
			}
		}
	}
	None
}

// * Utils to create a btree

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
	if last_char.is_ascii_uppercase() {
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
		match curr_formula.pop() {
			None => panic!("invalid formula : incomplete"),
			Some(c) => {
				if c.is_ascii_uppercase() {
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

// * Utils to print tree

pub fn print_tree(tree: &Tree<char>) {
	let mut tree_str = String::new();
	tree.write_formatted(&mut tree_str).unwrap();
	println!("Tree : \n{}", tree_str);
}

// Insert a node '!' just before node_id
pub fn insert_neg(tree: &mut Tree<char>, node_id : &NodeId) {
	let neg_id = tree.insert(Node::new('!'), UnderNode(node_id)).unwrap();
	tree.swap_nodes(node_id, &neg_id, TakeChildren).unwrap();
}


// * Utils to create subtree and insert them in a tree
// Create a whole subtree from origin_id node
pub fn copy_subtree(tree : & Tree<char>, origin_id : &NodeId) -> Tree<char>{
	let mut sub_tree = Tree::new();
	recurs_copy_subtree(tree, &mut sub_tree, origin_id, AsRoot);

	sub_tree
}

fn recurs_copy_subtree(tree : &Tree<char>, sub_tree : &mut Tree<char>, current_id : &NodeId, behavior : InsertBehavior) {
	let copy_id = sub_tree.insert(Node::new(*tree.get(current_id).unwrap().data()), behavior).unwrap();
	for i in 0..tree.get(current_id).unwrap().children().len() {
		let children_id = tree.get(current_id).unwrap().children().get(i).unwrap();
		recurs_copy_subtree(tree, sub_tree, children_id, UnderNode(&copy_id));
	}
}

// Insert sub_tree under node_id of tree
pub fn insert_subtree(tree : &mut Tree<char>, sub_tree : &Tree<char>, node_id  : &NodeId){
	recurs_insert_subtree(tree, sub_tree, node_id, sub_tree.root_node_id().unwrap());
}

fn recurs_insert_subtree(tree : &mut Tree<char>, sub_tree : &Tree<char>, destination_id : &NodeId, src_id : &NodeId) {
	let result_id = tree.insert(Node::new(*sub_tree.get(src_id).unwrap().data()), UnderNode(destination_id)).unwrap();
	for i in 0..sub_tree.get(src_id).unwrap().children().len() {
		let children_id = sub_tree.get(src_id).unwrap().children().get(i).unwrap();
		recurs_insert_subtree(tree, sub_tree, &result_id, children_id);
	}
}

#[cfg(test)]
	#[test]
	fn test_copy_subtree() {
		let mut tree = Tree::<char>::new();
		let node1 = Node::new('1');
		let node2 = Node::new('2');
		let node4 = Node::new('4');
		let node1_id = tree.insert(node1, AsRoot).unwrap();
		let node2_id = tree.insert(node2, UnderNode(&node1_id)).unwrap();
		let _node4_id = tree.insert(node4, UnderNode(&node2_id)).unwrap();
		let node3 = Node::new('3');
		let node3_id = tree.insert(node3, UnderNode(&node1_id)).unwrap();

		let mut tree_str = String::new();

		let sub_tree1 = copy_subtree(&tree, &node1_id);
		sub_tree1.write_formatted(&mut tree_str).unwrap();
		assert_eq!(tree_str,
"'1'
├── '2'
│   └── '4'
└── '3'
");

	tree_str.clear();
	let sub_tree2 = copy_subtree(&tree, &node2_id);
	sub_tree2.write_formatted(&mut tree_str).unwrap();
	assert_eq!(tree_str,
"'2'
└── '4'
");
		insert_subtree(&mut tree, &sub_tree2, &node3_id);
		tree_str.clear();
		tree.write_formatted(&mut tree_str).unwrap();
		assert_eq!(tree_str,
"'1'
├── '2'
│   └── '4'
└── '3'
    └── '2'
        └── '4'
"
);
	}
	#[test]
	fn test_already_valid00() {
		assert_eq!(negation_normal_form("A!B!C&D&|"), "A!B!C&D&|");
	}
	#[test]
	fn test_already_valid01() {
		assert_eq!(negation_normal_form("AB&C|"), "AB&C|");
	}
	#[test]
	fn test_neg_00() {
		assert_eq!(negation_normal_form("A!!"), "A");
	}
	#[test]
	fn test_neg_01() {
		assert_eq!(negation_normal_form("AB&!!"), "AB&");
	}
	#[test]
	fn test_and_00() {
		assert_eq!(negation_normal_form("AB&!"), "A!B!|");
	}
	#[test]
	fn test_and_01() {
		assert_eq!(negation_normal_form("A!B!&!"), "AB|");
	}
	#[test]
	fn test_and_02() {
		assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
	}
	#[test]
	fn test_or_00() {
		assert_eq!(negation_normal_form("AB|!"), "A!B!&");
	}
	#[test]
	fn test_or_01() {
		assert_eq!(negation_normal_form("AC&B|!"), "A!C!|B!&");
	}
	#[test]
	fn test_eq_00() {
		assert_eq!(negation_normal_form("AB="), "A!B!&AB&|");
	}
	#[test]
	fn test_eq_01() {
		assert_eq!(negation_normal_form("AB=!"), "AB|A!B!|&");
	}
	#[test]
	fn test_xor_00() {
		assert_eq!(negation_normal_form("AB^"), "A!B&AB!&|");
	}
	#[test]
	fn test_xor_01() {
		assert_eq!(negation_normal_form("AB!^"), "A!B!&AB&|");
	}
	#[test]
	fn test_impl_00() {
		assert_eq!(negation_normal_form("AB>"), "A!B|");
	}
	#[test]
	fn test_impl_01() {
		assert_eq!(negation_normal_form("AB>!"), "AB!&");
	}
	#[test]
	fn test_mix_00() {
		assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
	}

