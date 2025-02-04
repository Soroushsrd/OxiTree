// ** Tree of tree roots
// this tree is used to index and find the root of other trees.
// it attaches names to subvolumes and snapshots.
// and stores the location of extent allocation tree.
// It also stores the pointers to subvolumes and snapshots that are being deleted so that it could
// keep track!
//
// Superblock is stored at specific locations
pub const BTRFS_SUPER_INFO_OFFSET: u64 = 0x10000; // Primary superblock at 64KB
pub const BTRFS_SUPER_INFO_SIZE: usize = 4096; // One page/block
pub const BTRFS_DEFAULT_BLOCK_SIZE: usize = 16384; // 16 KB
use std::io::{Read, Seek, SeekFrom};

use crate::btrfs::{BtrfsInternalNode, BtrfsKey, BtrfsLeafNode};

pub struct BTree {
    pub root: Option<Node>,
    pub device: BlockDevice,
}

pub struct BlockDevice {
    pub handle: std::fs::File,
    pub size: usize,
}

#[derive(Clone)]
pub enum Node {
    Internal(BtrfsInternalNode),
    Leaf(BtrfsLeafNode),
}

impl BlockDevice {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let handle = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        let size = handle.metadata()?.len() as usize;
        Ok(BlockDevice { handle, size })
    }
    pub fn read_block(&mut self, block_ptr: u64) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0; self.size];
        self.handle
            .seek(SeekFrom::Start(block_ptr * self.size as u64))
            .expect("Failed to seek the block ptr");
        self.handle
            .read_exact(&mut buffer)
            .expect("Failed to read the data to buffer");
        Ok(buffer)
    }
}

impl BTree {
    pub fn new(device_path: &str) -> Result<Self, std::io::Error> {
        // /dev/sda2          # Second partition on first SATA drive
        // /dev/nvme0n1p1     # First partition on NVMe drive
        // /dev/loop0         # Loop device
        // for testing and development:
        // "./test_fs.img"    # Regular file simulating a block device
        // "/tmp/btrfs.img"   # Temporary filesystem image

        let mut device = BlockDevice::new(device_path)?;
        // In BTRFS, superblock is typically at block 0
        let superblock_data = device.read_block(0)?;
        let root_ptr = {
            let magic = &superblock_data[0..8];
            if magic != b"_BHRfs_M" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Not a valid BTRFS filesystem",
                ));
            }
            // Root tree pointer is at offset 0x68 (104 bytes) in superblock
            // we will use byte order conversion because BTRFS uses little-endian
            let ptr_bytes = &superblock_data[104..112];
            u64::from_le_bytes(ptr_bytes.try_into().unwrap())
        };
        todo!()
    }

    pub fn search<'a>(&'a self, key: &BtrfsKey) -> Option<&'a [u8]> {
        match &self.root {
            None => None,
            Some(node) => self.search_node(node, key),
        }
    }

    pub fn search_node<'a>(&'a self, node: &'a Node, search_key: &BtrfsKey) -> Option<&'a [u8]> {
        match node {
            Node::Leaf(leaf) => {
                match leaf.items.binary_search_by(|item| item.key.cmp(search_key)) {
                    Ok(idx) => {
                        let item = &leaf.items[idx];
                        let start = item.data_offset as usize;
                        let end = start + item.data_size as usize;
                        Some(&leaf.data[start..end])
                    }
                    Err(_) => None,
                }
            }
            Node::Internal(node) => {
                match node.keys.binary_search(search_key) {
                    Ok(idx) => {
                        // Key found - follow corresponding pointer
                        // Note: In a real implementation, you'd need to load the block
                        // pointed to by block_ptrs[idx]
                        todo!()
                    }
                    Err(idx) if idx > 0 => {
                        // Key not found - follow the pointer just before insertion point
                        // In a real implementation, you'd load the child node and continue searching
                        todo!()
                    }
                    Err(_) => None,
                }
            }
        }
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
