/// A simple in-memory AVL tree implementation.
use std::cmp::{Ord, Ordering};

type NodeRef<T> = Option<Box<AvlNode<T>>>;

#[derive(Eq, PartialEq, Debug)]
struct AvlNode<T: Ord> {
    key: T,
    height: u32,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AvlTree<T: Ord> {
    root: NodeRef<T>,
}

fn as_node_ref<T: Ord>(key: T) -> NodeRef<T> {
    Some(Box::new(AvlNode::new(key)))
}

impl<T: Ord> AvlNode<T> {
    fn new(key: T) -> Self {
        return AvlNode {
            key,
            height: 0,
            left: None,
            right: None,
        };
    }

    fn left_height(&self) -> Option<u32> {
        if let Some(ref left) = self.left {
            Some(left.height)
        } else {
            None
        }
    }

    fn right_height(&self) -> Option<u32> {
        if let Some(ref right) = self.right {
            Some(right.height)
        } else {
            None
        }
    }
}

impl<T: Ord> AvlTree<T> {
    /// Returns an ampty AVL tree.
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    /// Insert a value into the AVL tree, this operation runs in amortized O(log(n)).
    pub fn insert(&mut self, key: T) {
        let node_ref = &mut self.root;
        AvlTree::insert_rec(node_ref, key);
        /*
        let mut node_ref = &mut self.root;
        while let Some(ref mut node) = node_ref {
            match node.key.cmp(&key) {
                Ordering::Greater => node_ref = &mut node.left,
                Ordering::Less => node_ref = &mut node.right,
                Ordering::Equal => {
                    node.key = key;
                    return;
                }
            }
        }
        *node_ref = as_node_ref(key);
        */
    }

    /// Insert a value and return the node height.
    fn insert_rec(node_ref: &mut NodeRef<T>, key: T) -> u32 {
        if let Some(node) = node_ref {
            let (left_height, right_height) = match node.key.cmp(&key) {
                Ordering::Greater => (Some(AvlTree::insert_rec(&mut node.left, key)), node.right_height()),
                Ordering::Less => (node.left_height(), Some(AvlTree::insert_rec(&mut node.right, key))),
                Ordering::Equal => {
                    node.key = key;
                    (node.left_height(), node.right_height())
                }
            };
            // TODO: balance if necessary
            let height = if let (None, None) = (left_height, right_height) {
                0
            } else {
                1 + std::cmp::max(left_height.unwrap_or(0), right_height.unwrap_or(0))
            };
            node.height = height;
            height
        } else {
            *node_ref = as_node_ref(key);
            0
        }
    }

    pub fn get(&self, key: &T) -> Option<()> {
        let mut node_ref = &self.root;
        while let Some(ref node) = node_ref {
            match node.key.cmp(&key) {
                Ordering::Greater => node_ref = &node.left,
                Ordering::Less => node_ref = &node.right,
                Ordering::Equal => return Some(()),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = AvlTree::new();
        let target = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 1,
                height: 1,
                left: as_node_ref(0),
                right: as_node_ref(2),
            })),
        };
        tree.insert(1);
        tree.insert(0);
        tree.insert(2);
        assert_eq!(tree, target);
    }

    #[test]
    fn get() {
        let mut tree = AvlTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(0);
        tree.insert(5);

        assert!(tree.get(&0).is_some());
        assert!(tree.get(&1).is_some());
        assert!(tree.get(&2).is_some());
        assert!(tree.get(&5).is_some());
        assert!(tree.get(&4).is_none());
    }
}
