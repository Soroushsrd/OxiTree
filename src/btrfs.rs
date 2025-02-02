/// ***************************************************************************************
///Link for further info: https://btrfs.readthedocs.io/en/latest/dev/dev-btrfs-design.html*
/// ***************************************************************************************

/// Leaves have an array of fixed sized items and an area where items are stored.
///

/// Leaf Node (lvl 0)
/// Contains the actual Items
pub struct BtrfsLeafNode {
    pub header: BtrfsHeader,
    pub items: Vec<BtrfsItems>, // Almost the meta data about the item
    pub data: Vec<u8>,          // the actual data
}

/// Common to all nodes
/// The checksum of the lower node is not stored in the node pointer.
/// Generation number is known at the time the bblock is inserted into the btree,
/// Checksum is only calculated before writing the block to disk.
pub struct BtrfsHeader {
    pub checksum: u32,  // for data integrity
    pub fsid: [u8; 16], // file system identifier
    pub block_nr: u64,  // physical block number
    pub flags: u64,     // node type flags
    pub chunk_tree_uuid: [u8; 16],
    pub generation: u64, // transaction Id that allocated the block
    pub owner: u64,      //which tree node this blongs to
    pub nritems: u32,    //number of items in the node
    pub level: u8,       // tree level (0 for leaves)
}

#[allow(dead_code)]
/// The offset and and size fields in the items indicate where in the leaf the item can be found.
/// so for example nth item with the size of 10 would look like this:
/// |header|item 0|item 1|...|item N|Free space|data N|....|data 1|data 0|
///                            ^                   |
///                            |                   |
///     BtrfsItems{Key, data_offsent, datasize}    |--> the start of data N is data_offset and its
///                                                     length is data_size
///
pub struct BtrfsItems {
    pub key: BtrfsKey,
    data_offset: u32, // ? item index
    data_size: u32,   // item size
}

#[allow(dead_code)]
/// Key Structure
/// The offset field indicates the byte offset for a particular item in the object
/// for file extents, offset is the byte offset of the start of the extent in the file
pub struct BtrfsKey {
    object_id: u64, // identifies the object (file, directory, etc) allocated dynamically on creation
    type_id: u8,    // what kind of item is this (data, extent,directory)
    offset: u64,    //position within the object
}

/// Internal Nodes (lvl >0)
/// Contains keys and pointers to child nodes
pub struct BtrfsInternalNode {
    pub header: BtrfsHeader,
    pub keys: Vec<BtrfsKey>,  // used for searching
    pub block_ptrs: Vec<u64>, // points to child node
}

#[allow(non_camel_case_types)]
pub enum KeyTypes {
    INODE_ITEM = 0, // file meta data
    EXTENT_DATA,    // file data location
    DIR_ITEM,       // directory interies
    EXTENT_ITEM,    // extent meta data
    CHUNK_ITEM,     // block group info
}

// ** Inodes
// Inodes are stored in btrfs inode item at offset zero in their key and have type of value 0
// they store the stat data for files and directories

// ** Files
// small files that occupy less than one leaf block can be packet into btree inside the extent
// item. in this case their key offset is the byte offset of the data in the file and size
// indicates how much data they store.
//
// Larger files are stored in extents. btrfs file extent item records a generation for the extent
// and a [diskblock, disk num block] pair to indicate the disk area corresponding to file.
//
// file data checksum reflects the bytes sent to the disk so its calculated just before being sent

// ** Directories
// direcoties are indexed in two ways:
// 1. |Directory Objectid| BTRFS_DIR_ITEM_KEY| 64 bit filename hash|
// the default hash used is crc32c. a field flag in superblock will indicate which hash is used in
// FS field(?)
//
// 2. |Directory Objectid| BTRFS_DIR_ITEM_KEY| Inode sequence number|
// this better resembles the order of blocks on disk and gives a better performance.
// the Inode sequence number comes from the directory and each time a file is added, it increases
// by one

// *** Superblock
// it holds a pointer to the tree roots of the tree of tree roots and the chunk tree!

//////////////////////////
