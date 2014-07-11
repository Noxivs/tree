// The MIT License (MIT)
//
// Copyright (c) 2014 Noxivs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::{HashMap, Map};
use std::hash::{Hash};

/// A structure which represents a tree.
///
/// # Example
///
/// ```rust
/// fn main() {
/// 	let mut tree = Tree::<char,String>::new();
/// 	tree.insert_recursively(&vec!('A', 'B'), "Hello my nodes are A then B".into_string());
/// 	tree.insert_recursively(&vec!('A', 'C'), "Hello my nodes are A then C".into_string());
/// 
/// 	println!("{}", tree.find_recursively(&vec!('A', 'C')).unwrap()); #Hello my nodes are A then C
/// 	println!("{}", tree.find_recursively(&vec!('A', 'B')).unwrap()); #Hello my nodes are A then B
///		println!("{}", tree.find_recursively(&vec!('A', 'R')).unwrap()); #error
/// }
/// ```
pub struct Tree<K, V> {
	root: Box<TreeNode<K, V>>
}

impl<K: Eq + Hash + Clone, V> Tree<K, V> {
	/// Create an empty Tree.
	pub fn new() -> Tree<K, V> {
		Tree {
			root: box TreeNode::<K, V>::new(0)
		}
	}

	/// Return the value corresponding to the last valid key of the vector keys which 
	/// means that when a tree node contains a value on a current key, this value is returned.
	///
	/// Schema : 
	/// ```text
	///      A+---------------> Node without value, function will not return.
	///     + +
	///     | |        
	/// +---+ +---+        
	/// |         |        
	/// +         +        
	/// B:value1   C:value2---> Node with value, function will return it.
	/// +---------------------> Node with value, function will return it.           
	/// ```
	pub fn find_recursively<'a>(&'a self, keys: &Vec<K>)  -> Option<&'a V> {
		self.root.find_recursively(keys)
	}

	/// Insert or replace a value.
	pub fn insert_recursively(&mut self, keys: &Vec<K>, v: V) {
		self.root.insert_recursively(keys, v);
	}
}

/// A private structure which represents a node of the tree. The field `index` gives 
/// the position in the tree height. The field `children` contains all the tree nodes 
/// children.
struct TreeNode<K, V> {
	value: Option<V>,
	children: HashMap<K, Box<TreeNode<K, V>>>,
	index: i16
}

impl<K: Eq + Hash + Clone, V> TreeNode<K, V> {
	/// Create an empty TreeNode.
	fn new (index: i16) -> TreeNode<K, V> {
		TreeNode {
			value: None,
			children: HashMap::new(),
			index: index
		}
	}

	/// Return the value corresponding to the last valid key of the vector keys which 
	/// means that when a tree node contains a value on a current key, this value is returned.
	/// Schema : 
	///		cf the function `find_recursively` in the impl of the struct Tree.
	fn find_recursively<'a>(&'a self, keys: &Vec<K>)  -> Option<&'a V> {
		if self.value.is_some() {
			return Some(self.value.get_ref());
		}

		let k = keys.get(self.index as uint);
		match self.children.find(k) {
			Some(node) =>  node.find_recursively(keys),
			_ => None
		}
	}

	/// Insert or replace a value.
	fn insert_recursively(&mut self, keys: &Vec<K>, v: V) {

		if self.index - keys.len() as i16 >= 0 {
			self.value = Some(v);
		} else {
			let k = keys.get(self.index as uint);

			if !self.children.contains_key(k) {
				self.children.insert(k.clone(), box TreeNode::<K, V>::new(self.index + 1));
			}

			match self.children.find_mut(k) {
				Some(child)	=> child.insert_recursively(keys, v),
				_			=> { }
			}
		}
	}
}

/* TODO : Implement traits

impl<K: Eq + Hash, V> Collection for TreeNode<K, V> {
	fn len(&self) -> uint { 
		self.children.len()
	}

	fn is_empty(&self) -> bool { 
		self.len() == 0 
	}
}

impl<K: Eq + Hash, V> Map<K, V> for TreeNode<K, V> {
	fn contains_key(&self, k: &K) -> bool {
		self.find(k).is_some()
	}

	fn find<'a>(&'a self, k: &K)  -> Option<&'a V> {
		match self.children.find(k)  {
			Some(node) => 
				match node.value {
					Some(ref v) => Some(v),
					_ => None
				},
			_ => None
		}
	}
}

*/