// ** Tree of tree roots
// this tree is used to index and find the root of other trees.
// it attaches names to subvolumes and snapshots.
// and stores the location of extent allocation tree.
// It also stores the pointers to subvolumes and snapshots that are being deleted so that it could
// keep track!
//

use crate::btrfs::{BtrfsInternalNode, BtrfsKey, BtrfsLeafNode};

pub struct BTree {
    pub root: Option<Node>,
}

pub enum Node {
    Internal(BtrfsInternalNode),
    Leaf(BtrfsLeafNode),
}

impl BTree {
    pub fn new() -> Self {
        todo!()
    }

    pub fn search(&self, key: &BtrfsKey) -> Option<&[u8]> {
        todo!()
    }

    pub fn search_node(&self, node: &Box<Node>, searchKey: &BtrfsKey) -> Option<&[u8]> {
        todo!()
    }
    /// To insert items into a tree
    pub fn insert() {
        todo!()
    }
    /// To remove items from a tree
    pub fn delete() {
        todo!()
    }

    /// To create an internal or a leaf node
    pub fn create_node() {
        todo!()
    }
    /// To fetch data using block pointers
    pub fn read_node() {
        todo!()
    }
    /// To persist node changes to disk
    pub fn write_node() {
        todo!()
    }
    /// To merge nodes when they become too empty
    pub fn merge_node() {
        todo!()
    }
    /// To split nodes when they become too full!
    pub fn split_node() {
        todo!()
    }
}
