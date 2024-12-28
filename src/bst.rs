// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.

use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug)]
pub enum TreeErr {
    NoValue,
    ValueAlreadyExistInTree,
    EmptyTree,
}

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

    /// Returns a tree containing a single value
    pub fn leaf(value: T) -> Self {
        Tree(Some(Box::new({
            Node { value, left:Tree(None), right: Tree(None) }
        })))
    }

    /// Returns true if and only if `value` belongs to the tree.
    pub fn contains(&self, value: T) -> bool{
        match self.0 {
            Some(ref node )=> match value.cmp(&node.value){
                Ordering::Equal=>true , 
                Ordering::Greater=> node.right.contains(value),
                Ordering::Less=> node.left.contains(value),
            },
            None => false , 
        }
    }

    /// Inserts `value` into the tree.
    /// Returns `TreeErr::ValueAlreadyExistInTree` iff the `value` was already contained in the tree.
    pub fn insert(&mut self, value: T) -> Result<(),TreeErr>{
        match self.0 {
            Some(ref mut node)=> match value.cmp(&node.value){
                Ordering::Equal=> Err(TreeErr::ValueAlreadyExistInTree),
                Ordering::Greater=> node.right.insert(value),
                Ordering::Less=> node.left.insert(value),
            },
            None=> {
                *self=Tree::leaf(value);
                Ok(())
            }
        }

    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_empty_tree(){
        let tree:Tree<i32>=Tree::new(); 
        assert!(tree.0.is_none());

    }
    #[test]
    fn test_tree_with_a_single_value(){
        let tree=Tree::leaf(13);
        assert!(tree.0.is_some());
        if let Some(node)=&tree.0{
            assert_eq!(node.value,13);
            assert!(node.left.0.is_none());
            assert!(node.right.0.is_none());
        }

    }
}