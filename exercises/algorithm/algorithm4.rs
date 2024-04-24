/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut root = &mut self.root;
        while let Some(node) = root {
            match node.value.cmp(&value) {
                Ordering::Greater => {
                    root = &mut node.left;
                }
                Ordering::Less => {
                    root = &mut node.right;
                }
                _ => {
                    return;
                }
            }
        }

        *root = Some(Box::new(TreeNode::new(value.clone())));
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut root = &self.root;
        while let Some(node) = root {
            match node.value.cmp(&value) {
                Ordering::Greater => {
                    root = &node.left;
                }
                Ordering::Less => {
                    root = &node.right;
                }
                _ => {
                    return true;
                }
            }
        }

        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        *self = TreeNode::new(value.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


