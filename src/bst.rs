// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.

use std::{cmp::Ordering, fmt::Debug};

#[derive(Debug)]
pub enum TreeErr {
    NoValue,
    ValueAlreadyExistInTree,
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

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `TreeErr::NoValue` is returned.
    pub fn delete(&mut self, value: T)-> Result<(),TreeErr>{
        match self.0 {
            Some(ref mut node )=>match value.cmp(&node.value){
                Ordering::Equal=> {
                    // search for his closest successor..
                    match node.right.find_and_remove_successor(){
                        Some(succ_value)=>node.value=succ_value,
                        None=>self.0=node.left.0.take(),
                    }
                    Ok(())
                },
                Ordering::Greater=>node.right.delete(value),
                Ordering::Less=> node.left.delete(value),
            }, 
            None=> Err(TreeErr::NoValue),
        }
    }

    // Returns the value of successor or None 
    pub fn find_and_remove_successor(&mut self)-> Option<T>{
        match self.0 {
            Some(ref mut node)=>match node.left.find_and_remove_successor(){
                Some(left_child)=>Some(left_child),
                None=>{
                    let succ =self.0.take().unwrap();
                    self.0=succ.right.0;
                    Some(succ.value)
                }
            },
            None=>None,
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
    #[test]
    fn test_tree_insert_and_contains_values(){
        let mut tree =Tree::new();
        tree.insert(19).expect("Insertion Impossible");
        tree.insert(7).expect("Insertion Impossible");
        tree.insert(23).expect("Insertion Impossible");
        assert!(tree.insert(23).is_err());//Err car 23 est deja present dans l'abr
        tree.insert(32).expect("Insertion Impossible");
        assert!(tree.contains(19));assert!(tree.contains(7));assert!(tree.contains(23));assert!(tree.contains(32));
        assert!(!tree.contains(5));
    }
    #[test]
    fn test_delete_value_from_empty_tree(){
        let mut tree =Tree::new();
        assert!(tree.delete(3).is_err());
    }
    #[test]
    fn test_delete(){
        let mut tree =Tree::new();
        tree.insert(15).expect("Insertion Impossible");tree.insert(12).expect("Insertion Impossible");
        tree.insert(19).expect("Insertion Impossible");tree.insert(17).expect("Insertion Impossible");
        assert!(tree.delete(15).is_ok());
    }
}