// Tests related to AVL Tree correctness

#[cfg(test)]
mod tests {
    use crate::avl::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn insert() {
        let data = [42];
        let mut tree = AvlTree::new();
        let target = AvlTree {
            root: build_node(1, data, as_node_ref(0, data), as_node_ref(2, data)),
        };
        tree.insert(1, data);
        tree.insert(0, data);
        tree.insert(2, data);
        assert_eq!(tree, target);
    }

    #[test]
    fn get() {
        let mut tree = AvlTree::new();
        tree.insert(1, [1]);
        tree.insert(2, [2]);
        tree.insert(0, [0]);
        tree.insert(5, [5]);

        assert_eq!(tree.get(&0), Some(&[0]));
        assert_eq!(tree.get(&1), Some(&[1]));
        assert_eq!(tree.get(&2), Some(&[2]));
        assert_eq!(tree.get(&5), Some(&[5]));
        assert_eq!(tree.get(&4), None);
    }

    #[test]
    fn rotate_right() {
        let mut before = AvlTree {
            root: build_node(
                5,
                [5],
                build_node(3, [3], as_node_ref(2, [2]), as_node_ref(4, [4])),
                as_node_ref(6, [6]),
            ),
        };
        let after = AvlTree {
            root: build_node(
                3,
                [3],
                as_node_ref(2, [2]),
                build_node(5, [5], as_node_ref(4, [4]), as_node_ref(6, [6])),
            ),
        };
        AvlTree::rotate_right(&mut before.root);
        assert_eq!(before, after);
    }

    #[test]
    fn rotate_left() {
        let mut before = AvlTree {
            root: build_node(
                1,
                [1],
                as_node_ref(0, [0]),
                build_node(3, [3], as_node_ref(2, [2]), as_node_ref(4, [4])),
            ),
        };
        let after = AvlTree {
            root: build_node(
                3,
                [3],
                build_node(1, [1], as_node_ref(0, [0]), as_node_ref(2, [2])),
                as_node_ref(4, [4]),
            ),
        };
        AvlTree::rotate_left(&mut before.root);
        assert_eq!(before, after);
    }

    #[test]
    fn integration() {
        let mut tree = AvlTree::new();
        tree.insert('M', [0]);
        tree.insert('N', [0]);
        tree.insert('O', [0]);
        tree.insert('L', [0]);
        tree.insert('K', [0]);
        tree.insert('Q', [0]);
        tree.insert('P', [0]);
        tree.insert('H', [0]);
        tree.insert('I', [0]);
        tree.insert('A', [0]);
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

    /// Hash a single u8, for convenience.
    fn hash_int(value: u8) -> Hash {
        let hash = Sha256::digest(&[value]);
        Hash::from_bytes(HASH_ALGO, &hash).unwrap()
    }

    /// An helper function to build simple AvlNodes.
    fn build_node<T: Ord>(
        key: T,
        value: [u8; 1],
        left: NodeRef<T, [u8; 1]>,
        right: NodeRef<T, [u8; 1]>,
    ) -> NodeRef<T, [u8; 1]> {
        let hash = hash_int(value[0]);
        let mut height = 0;
        let mut sha = Sha256::new();
        let merkle_hash = if let (None, None) = (&left, &right) {
            hash
        } else {
            if let Some(ref left) = left {
                sha.update(left.merkle_hash.as_bytes());
                height = left.height + 1;
            }
            if let Some(ref right) = right {
                sha.update(right.merkle_hash.as_bytes());
                height = std::cmp::max(right.height + 1, height);
            }
            sha.update(hash.as_bytes());
            Hash::from_bytes(HASH_ALGO, sha.finalize().as_slice()).unwrap()
        };
        Some(Box::new(AvlNode {
            key,
            value,
            hash,
            merkle_hash,
            height,
            left,
            right,
        }))
    }
}
