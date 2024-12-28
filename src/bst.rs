// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.

use std::fmt::Debug;
#[derive(Debug)]
pub struct Tree<T>(Option<Box<Node<T>>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl <T:Ord +Debug> Tree <T>{
    /// Returns an empty tree
    pub fn new() -> Self {
        Tree(None)
    }
}