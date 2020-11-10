// Tests related to AVL Tree correctness

#[cfg(test)]
mod tests {
    use crate::avl::*;

    #[test]
    fn insert() {
        let mut tree = AvlTree::new();
        let target = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 1,
                value: 1,
                height: 1,
                left: as_node_ref(0, 0),
                right: as_node_ref(2, 2),
            })),
        };
        tree.insert(1, 1);
        tree.insert(0, 0);
        tree.insert(2, 2);
        assert_eq!(tree, target);
    }

    #[test]
    fn get() {
        let mut tree = AvlTree::new();
        tree.insert(1, 1);
        tree.insert(2, 2);
        tree.insert(0, 0);
        tree.insert(5, 5);

        assert_eq!(tree.get(&0), Some(&0));
        assert_eq!(tree.get(&1), Some(&1));
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&4), None);
    }

    #[test]
    fn rotate_right() {
        let mut before = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 5,
                value: 5,
                height: 2,
                left: Some(Box::new(AvlNode {
                    key: 3,
                    value: 3,
                    height: 1,
                    left: as_node_ref(2, 2),
                    right: as_node_ref(4, 4),
                })),
                right: as_node_ref(6, 6),
            })),
        };
        let after = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 3,
                value: 3,
                height: 2,
                left: as_node_ref(2, 2),
                right: Some(Box::new(AvlNode {
                    key: 5,
                    value: 5,
                    height: 1,
                    left: as_node_ref(4, 4),
                    right: as_node_ref(6, 6),
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
                value: 1,
                height: 2,
                left: as_node_ref(0, 0),
                right: Some(Box::new(AvlNode {
                    key: 3,
                    value: 3,
                    height: 1,
                    left: as_node_ref(2, 2),
                    right: as_node_ref(4, 4),
                })),
            })),
        };
        let after = AvlTree {
            root: Some(Box::new(AvlNode {
                key: 3,
                value: 3,
                height: 2,
                left: Some(Box::new(AvlNode {
                    key: 1,
                    value: 1,
                    height: 1,
                    left: as_node_ref(0, 0),
                    right: as_node_ref(2, 2),
                })),
                right: as_node_ref(4, 4),
            })),
        };
        AvlTree::rotate_left(&mut before.root);
        assert_eq!(before, after);
    }

    #[test]
    fn integration() {
        let mut tree = AvlTree::new();
        tree.insert('M', ());
        tree.insert('N', ());
        tree.insert('O', ());
        tree.insert('L', ());
        tree.insert('K', ());
        tree.insert('Q', ());
        tree.insert('P', ());
        tree.insert('H', ());
        tree.insert('I', ());
        tree.insert('A', ());
        assert!(check_integrity(&tree.root));
    }

    /// Check that nodes are ordered, heights are correct and that balance factors are in {-1, 0, 1}.
    fn check_integrity<T: Ord, V>(node_ref: &NodeRef<T, V>) -> bool {
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
