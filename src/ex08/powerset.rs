
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
	let mut powerset = Vec::<Vec::<i32>>::new();
	for i in 0..(1 << set.len()) {
		let mut current_set = Vec::<i32>::new();
		for (pos, item_set) in set.iter().enumerate() {
			if i >> pos & 1 == 1 {
				current_set.push(*item_set);
			}
		}
		powerset.push(current_set);
	}

	powerset
}

#[cfg(test)]
	#[test]
	fn test_00() {
		assert_eq!(powerset(vec![]), vec![vec![]]);
		assert_eq!(powerset(vec![]), vec![[]]);
		assert_eq!(powerset(vec![]), vec![[]]);
	}
	#[test]
	fn test_01() {
		assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
	}
	#[test]
	fn test_02() {
		assert_eq!(powerset(vec![1,2]), vec![vec![], vec![1], vec![2], vec![1, 2]]);
	}
	#[test]
	fn test_03() {
		assert_eq!(powerset(vec![1,2,3]), vec![vec![], vec![1], vec![2], vec![1, 2],
											   vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
	}