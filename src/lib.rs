use std::iter;

pub trait Tree: Sized {
	type Iter<'a, T>: Iterator<Item=&'a T> where T: 'a, Self: 'a;
	fn children<'a>(&'a self) -> Self::Iter<'a, Self>;

	fn df_traverse<'a>(&'a self) -> TreeIter<'a, Self> {
		TreeIter {
			stack: iter::once(self).collect()
		}
	}
}

pub struct TreeIter<'a, T: Tree> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for TreeIter<'a, T> where T: Tree {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		let next = match self.stack.pop() {
			Some(node) => node,
			None => return None,
		};
		self.stack.extend(next.children());
		Some(next)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug)]
	struct TestTree {
		value: u32,
		children: Vec<TestTree>
	}
	impl Tree for TestTree {
		type Iter<'a, T> = std::slice::Iter<'a, T> where T: 'a;
		fn children<'a>(&'a self) -> Self::Iter<'a, Self> {
			self.children.iter()
		}
	}
	impl PartialEq<u32> for &TestTree {
		fn eq(&self, other: &u32) -> bool {
			self.value == *other
		}
	}

	#[test]
	fn df_traversal() {
		let tree = TestTree {
			value: 0,
			children: vec![
				TestTree {
					value: 4,
					children: vec![
						TestTree {
							value: 5,
							children: vec![],
						},
					],
				},
				TestTree {
					value: 1,
					children: vec![
						TestTree {
							value: 3,
							children: vec![],
						},
						TestTree {
							value: 2,
							children: vec![],
						},
					],
				}
			],
		};
		let result: Vec<_> = tree.df_traverse().collect();
		assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);
	}
}
