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

    /// The left height, or None if there is no left child.
    fn left_height(&self) -> Option<u32> {
        if let Some(ref left) = self.left {
            Some(left.height)
        } else {
            None
        }
    }

    /// The right height, or None if there is no right child.
    fn right_height(&self) -> Option<u32> {
        if let Some(ref right) = self.right {
            Some(right.height)
        } else {
            None
        }
    }

    /// Update the hight of the node by looking at the hight of its two children.
    fn update_height(&mut self) {
        match &self.right {
            None => match &self.left {
                None => self.height = 0,
                Some(left) => self.height = left.height + 1,
            },
            Some(right) => match &self.left {
                None => self.height = right.height + 1,
                Some(left) => self.height = std::cmp::max(left.height, right.height) + 1,
            },
        }
    }

    /// Returns the node's balance factor (left_height - right_height).
    fn balance_factor(&self) -> i32 {
        match (self.left_height(), self.right_height()) {
            (None, None) => 0,
            (None, Some(h)) => -(h as i32),
            (Some(h), None) => h as i32,
            (Some(h_l), Some(h_r)) => (h_l as i32) - (h_r as i32),
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
    }

    /// Insert a value and return the node height.
    fn insert_rec(node_ref: &mut NodeRef<T>, key: T) {
        if let Some(node) = node_ref {
            match node.key.cmp(&key) {
                Ordering::Greater => AvlTree::insert_rec(&mut node.left, key),
                Ordering::Less => AvlTree::insert_rec(&mut node.right, key),
                Ordering::Equal => node.key = key,
            }
            node.update_height();
            AvlTree::balance_node(node_ref);
        } else {
            *node_ref = as_node_ref(key);
        }
    }

    fn balance_node(node_ref: &mut NodeRef<T>) {
        let node = node_ref
            .as_mut()
            .expect("[AVL]: Empty node in node balance");
        let balance_factor = node.balance_factor();
        if balance_factor >= 2 {
            let left = node
                .left
                .as_mut()
                .expect("[AVL]: Unexpected empty left node");
            if left.balance_factor() >= 1 {
                AvlTree::rotate_right(node_ref);
            } else {
                AvlTree::rotate_left(&mut node.left);
                AvlTree::rotate_right(node_ref);
            }
        } else if balance_factor <= -2 {
            let right = node
                .right
                .as_mut()
                .expect("[AVL]: Unexpected empty right node");
            if right.balance_factor() <= -1 {
                AvlTree::rotate_left(node_ref);
            } else {
                AvlTree::rotate_right(&mut node.right);
                AvlTree::rotate_left(node_ref);
            }
        }
    }

    /// Performs a right rotation.
    fn rotate_right(root: &mut NodeRef<T>) {
        let mut node = root.take().expect("[AVL]: Empty root in right rotation");
        let mut left = node.left.take().expect("[AVL]: Unexpected right rotation");
        let mut left_right = left.right.take();
        std::mem::swap(&mut node.left, &mut left_right);
        node.update_height();
        std::mem::swap(&mut left.right, &mut Some(node));
        left.update_height();
        std::mem::swap(root, &mut Some(left));
    }

    /// Perform a left rotation.
    fn rotate_left(root: &mut NodeRef<T>) {
        let mut node = root.take().expect("[AVL]: Empty root in left rotation");
        let mut right = node.right.take().expect("[AVL]: Unexpected left rotation");
        let mut right_left = right.left.take();
        std::mem::swap(&mut node.right, &mut right_left);
        node.update_height();
        std::mem::swap(&mut right.left, &mut Some(node));
        right.update_height();
        std::mem::swap(root, &mut Some(right))
    }

    /// Return the value corresponding to the key, if it exists.
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

    #[test]
    fn rotate_right() {
        let mut before = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 5,
                height: 2,
                left: Some(Box::new(AvlNode {
                    key: 3,
                    height: 1,
                    left: as_node_ref(2),
                    right: as_node_ref(4),
                })),
                right: as_node_ref(6),
            })),
        };
        let after = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 3,
                height: 2,
                left: as_node_ref(2),
                right: Some(Box::new(AvlNode {
                    key: 5,
                    height: 1,
                    left: as_node_ref(4),
                    right: as_node_ref(6),
                })),
            })),
        };
        AvlTree::rotate_right(&mut before.root);
        assert_eq!(before, after);
    }

    #[test]
    fn rotate_left() {
        let mut before = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 1,
                height: 2,
                left: as_node_ref(0),
                right: Some(Box::new(AvlNode {
                    key: 3,
                    height: 1,
                    left: as_node_ref(2),
                    right: as_node_ref(4),
                })),
            })),
        };
        let after = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 3,
                height: 2,
                left: Some(Box::new(AvlNode {
                    key: 1,
                    height: 1,
                    left: as_node_ref(0),
                    right: as_node_ref(2),
                })),
                right: as_node_ref(4),
            })),
        };
        AvlTree::rotate_left(&mut before.root);
        assert_eq!(before, after);
    }

    #[test]
    fn integration() {
        let mut tree = AvlTree::new();
        tree.insert('M');
        tree.insert('N');
        tree.insert('O');
        tree.insert('L');
        tree.insert('K');
        tree.insert('Q');
        tree.insert('P');
        tree.insert('H');
        tree.insert('I');
        tree.insert('A');
        assert!(check_integrity(&tree.root));
    }

    /// Check that nodes are ordered, heights are correct and that balance factors are in {-1, 0, 1}.
    fn check_integrity<T: Ord>(node_ref: &NodeRef<T>) -> bool {
        if let Some(node) = node_ref {
            let mut left_height = 0;
            let mut right_height = 0;
            let mut is_leaf = true;
            if let Some(ref left) = node.left {
                if left.key >= node.key {
                    println!("[AVL]: Left child should have a smaller key");
                    return false;
                }
                left_height = left.height;
                is_leaf = false;
            }
            if let Some(ref right) = node.right {
                if right.key <= node.key {
                    println!("[AVL]: Right child should have a bigger key");
                    return false;
                }
                right_height = right.height;
                is_leaf = false;
            }
            let balance_factor = (left_height as i32) - (right_height as i32);
            if balance_factor <= -2 {
                println!("[AVL] Balance factor <= -2");
                return false;
            } else if balance_factor >= 2 {
                println!("[AVL] Balance factor >= 2");
                return false;
            }
            let bonus_height = if is_leaf { 0 } else { 1 };
            if node.height != std::cmp::max(left_height, right_height) + bonus_height {
                println!("[AVL] Heights are inconsistent");
                return false;
            }
            return check_integrity(&node.left) && check_integrity(&node.right);
        } else {
            true
        }
    }
}
