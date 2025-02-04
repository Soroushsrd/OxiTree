use std::cmp::Ordering;

/// ***************************************************************************************
///Link for further info: https://btrfs.readthedocs.io/en/latest/dev/dev-btrfs-design.html*
/// ***************************************************************************************

/// Leaves have an array of fixed sized items and an area where items are stored.
///

/// Leaf Node (lvl 0)
/// Contains the actual Items
#[derive(Clone, Debug)]
pub struct BtrfsLeafNode {
    pub header: BtrfsHeader,
    pub items: Vec<BtrfsItems>, // Almost the meta data about the item
    pub data: Vec<u8>,          // the actual data
}

/// Common to all nodes
/// The checksum of the lower node is not stored in the node pointer.
/// Generation number is known at the time the bblock is inserted into the btree,
/// Checksum is only calculated before writing the block to disk.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct BtrfsItems {
    pub key: BtrfsKey,
    pub data_offset: u32, // ? item index
    pub data_size: u32,   // item size
}

#[allow(dead_code)]
/// Key Structure
/// The offset field indicates the byte offset for a particular item in the object
/// for file extents, offset is the byte offset of the start of the extent in the file
#[derive(Clone, Debug)]
pub struct BtrfsKey {
    object_id: u64, // identifies the object (file, directory, etc) allocated dynamically on creation
    type_id: u8,    // what kind of item is this (data, extent,directory)
    offset: u64,    //position within the object
}

/// Internal Nodes (lvl >0)
/// Contains keys and pointers to child nodes
#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
pub struct BtrfsSuperblock {
    // Magic number for BTRFS_MAGIC: _BHRfS_M (0x4D5F53665248425F)
    pub magic: u64,

    // Core properties
    pub checksum: [u8; 0x20], // Checksum of everything past this field
    pub fsid: [u8; 0x10],     // Filesystem UUID
    pub bytenr: u64,          // Physical address of this block
    pub flags: u64,           // Superblock flags

    // Header info
    pub generation: u64, // Transaction ID that wrote this superblock
    pub root: u64,       // Root tree root
    pub chunk_root: u64, // Chunk tree root
    pub log_root: u64,   // Log tree root
    pub log_root_transid: u64,

    // Global filesystem properties
    pub total_bytes: u64,          // Total bytes used
    pub bytes_used: u64,           // Bytes used in filesystem
    pub root_dir_objectid: u64,    // ObjectID of root directory (usually 6)
    pub num_devices: u64,          // Number of devices in filesystem
    pub sectorsize: u32,           // Sector size in bytes
    pub nodesize: u32,             // Node size in bytes
    pub leafsize: u32,             // Leaf size in bytes
    pub stripesize: u32,           // Stripe size in bytes
    pub sys_chunk_array_size: u32, // Size of system chunk array
    pub chunk_root_generation: u64,

    // Version information
    pub compat_flags: u64,    // Compatible feature flags
    pub compat_ro_flags: u64, // Read-only compatible feature flags
    pub incompat_flags: u64,  // Incompatible feature flags

    // Checksumming
    pub csum_type: u16,       // Checksum algorithm type
    pub root_level: u8,       // Root tree level
    pub chunk_root_level: u8, // Chunk tree level
    pub log_root_level: u8,   // Log tree level

    // Device information
    pub dev_item: BtrfsDevItem, // Device information

    // Label and backup information
    pub label: [u8; 0x100],        // Filesystem label
    pub cache_generation: u64,     // Generation of cached blocks
    pub uuid_tree_generation: u64, // UUID tree generation

    // Reserved space and metadata
    pub reserved: [u8; 0xf0],         // Reserved for future expansion
    pub sys_chunk_array: [u8; 0x800], // System chunk array for bootstrapping
    pub super_roots: [u8; 0x2a0],
    pub unused: [u8; 0x235],
}

/// DevItem contains information about a single device in the Btrfs filesystem.
/// The layout follows the Btrfs specification exactly, with each field at its specified offset.
#[derive(Default, Debug, Clone)]
pub struct BtrfsDevItem {
    pub devid: u64,            // 0x00-0x08: Unique device identifier
    pub total_bytes: u64,      // 0x08-0x10: Total size of the device
    pub bytes_used: u64,       // 0x10-0x18: Number of bytes used on device
    pub io_align: u32,         // 0x18-0x1c: Optimal I/O alignment
    pub io_width: u32,         // 0x1c-0x20: Optimal I/O width
    pub sector_size: u32,      // 0x20-0x24: Minimal I/O size (sector size)
    pub dev_type: u64,         // 0x24-0x2c: Device type
    pub generation: u64,       // 0x2c-0x34: Generation when device was added
    pub start_offset: u64,     // 0x34-0x3c: Start offset
    pub dev_group: u32,        // 0x3c-0x40: Device group
    pub seek_speed: u8,        // 0x40-0x41: Seek speed (1-100)
    pub bandwidth: u8,         // 0x41-0x42: Bandwidth (1-100)
    pub device_uuid: [u8; 16], // 0x42-0x52: Device UUID
    pub fsid: [u8; 16],        // 0x52-0x62: Filesystem UUID
}
impl BtrfsDevItem {
    /// Deserializes a DevItem from a byte buffer.
    /// The buffer must be at least 0x62 bytes long.
    pub fn read_from_buff(&mut self, buffer: &[u8]) -> Result<Self, std::io::Error> {
        if buffer.len() < 0x62 {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        let read_u64 = |slice: &[u8]| -> u64 {
            u64::from_le_bytes(slice.try_into().expect("failed to read u64"))
        };

        let read_u32 = |slice: &[u8]| -> u32 {
            u32::from_le_bytes(slice.try_into().expect("failed at reading u32"))
        };

        let mut device_uuid = [0u8; 16];
        device_uuid.copy_from_slice(&buffer[0x42..0x52]);

        let mut fsid = [0u8; 16];
        fsid.copy_from_slice(&buffer[0x52..0x62]);

        Ok(BtrfsDevItem {
            devid: read_u64(&buffer[..0x08]),
            total_bytes: read_u64(&buffer[0x08..0x10]),
            bytes_used: read_u64(&buffer[0x10..0x18]),
            io_align: read_u32(&buffer[0x18..0x1c]),
            io_width: read_u32(&buffer[0x1c..0x20]),
            sector_size: read_u32(&buffer[0x20..0x24]),
            dev_type: read_u64(&buffer[0x24..0x2c]),
            generation: read_u64(&buffer[0x2c..0x34]),
            start_offset: read_u64(&buffer[0x34..0x3c]),
            dev_group: read_u32(&buffer[0x3c..0x40]),
            seek_speed: buffer[0x40],
            bandwidth: buffer[0x41],
            device_uuid,
            fsid,
        })
    }
}

/// The sys_chunk_array contains pairs of (Key, ChunkItem)
/// Each pair describes a system chunk's logical and physical mapping
#[derive(Debug, Clone)]
pub struct BtrfsChunkItem {
    pub size: u64,                      // Size of the chunk
    pub owner: u64,                     // Object ID of the owner (EXTENT_TREE_OBJECTID)
    pub stripe_len: u64,                // Stripe length
    pub type_: u64,                     // Type of the chunk (SYSTEM, DATA, METADATA)
    pub io_align: u32,                  // IO alignment requirement
    pub io_width: u32,                  // IO width requirement
    pub sector_size: u32,               // Sector size
    pub num_stripes: u16,               // Number of stripes
    pub sub_stripes: u16,               // Number of sub-stripes
    pub stripes: Vec<BtrfsChunkStripe>, // Array of stripe information
}

/// Each stripe represents a piece of a chunk on a physical device
#[derive(Debug, Clone)]
pub struct BtrfsChunkStripe {
    pub devid: u64,         // Device ID
    pub offset: u64,        // Offset on the device
    pub dev_uuid: [u8; 16], // UUID of the device
}

impl BtrfsSuperblock {
    const MAGIC: &'static [u8; 8] = b"_BHRfS_M";

    pub fn from_buffer(buffer: &[u8]) -> Result<Self, &'static str> {
        if buffer.len() < 0x1000 {
            return Err("Buffer too small for superblock");
        }

        // Helper closures for reading integers
        let read_u64 = |slice: &[u8]| -> u64 { u64::from_le_bytes(slice.try_into().unwrap()) };

        let read_u32 = |slice: &[u8]| -> u32 { u32::from_le_bytes(slice.try_into().unwrap()) };

        let read_u16 = |slice: &[u8]| -> u16 { u16::from_le_bytes(slice.try_into().unwrap()) };

        // Read dev_item first since we'll need it for the struct initialization
        let mut dev_item = BtrfsDevItem::default();
        BtrfsDevItem::read_from_buff(&mut dev_item, &buffer[0xc9..0x12b]).unwrap();

        let mut superblock = Self {
            checksum: buffer[0x00..0x20].try_into().unwrap(),
            fsid: buffer[0x20..0x30].try_into().unwrap(),
            bytenr: read_u64(&buffer[0x30..0x38]),
            flags: read_u64(&buffer[0x38..0x40]),
            magic: read_u64(&buffer[0x40..0x48]),
            generation: read_u64(&buffer[0x48..0x50]),
            root: read_u64(&buffer[0x50..0x58]),
            chunk_root: read_u64(&buffer[0x58..0x60]),
            log_root: read_u64(&buffer[0x60..0x68]),
            log_root_transid: read_u64(&buffer[0x68..0x70]),
            total_bytes: read_u64(&buffer[0x70..0x78]),
            bytes_used: read_u64(&buffer[0x78..0x80]),
            root_dir_objectid: read_u64(&buffer[0x80..0x88]),
            num_devices: read_u64(&buffer[0x88..0x90]),
            sectorsize: read_u32(&buffer[0x90..0x94]),
            nodesize: read_u32(&buffer[0x94..0x98]),
            leafsize: read_u32(&buffer[0x98..0x9c]),
            stripesize: read_u32(&buffer[0x9c..0xa0]),
            sys_chunk_array_size: read_u32(&buffer[0xa0..0xa4]),
            chunk_root_generation: read_u64(&buffer[0xa4..0xac]),
            compat_flags: read_u64(&buffer[0xac..0xb4]),
            compat_ro_flags: read_u64(&buffer[0xb4..0xbc]),
            incompat_flags: read_u64(&buffer[0xbc..0xc4]),
            csum_type: read_u16(&buffer[0xc4..0xc6]),
            root_level: buffer[0xc6],
            chunk_root_level: buffer[0xc7],
            log_root_level: buffer[0xc8],
            dev_item,
            label: buffer[0x12b..0x22b].try_into().unwrap(),
            cache_generation: read_u64(&buffer[0x22b..0x233]),
            uuid_tree_generation: read_u64(&buffer[0x233..0x23b]),
            reserved: buffer[0x23b..0x32b].try_into().unwrap(),
            sys_chunk_array: buffer[0x32b..0xb2b].try_into().unwrap(),
            super_roots: buffer[0xb2b..0xdcb].try_into().unwrap(),
            unused: buffer[0xdcb..0x1000].try_into().unwrap(),
        };

        Ok(superblock)
    }

    /// Verifies the integrity of the superblock by checking:
    /// 1. Magic number
    /// 2. Checksum (CRC32c with seed -1)
    /// 3. Basic sanity checks on values
    pub fn verify(&self) -> bool {
        let read_u64 = |slice: &[u8]| -> u64 { u64::from_le_bytes(slice.try_into().unwrap()) };

        if &self.magic != &read_u64(Self::MAGIC) {
            return false;
        }

        if !self.sectorsize.is_power_of_two() {
            return false;
        }

        // Verify node size is a power of 2
        if !self.nodesize.is_power_of_two() {
            return false;
        }

        // Verify dev_item's fsid matches superblock's fsid
        if self.fsid != self.dev_item.fsid {
            return false;
        }

        // TODO: Implement checksum verification
        // The checksum is calculated over everything after the checksum field
        // using CRC32c with seed -1
        todo!()
        // true
    }
}

impl PartialOrd for BtrfsKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BtrfsKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare object_id first
        match self.object_id.cmp(&other.object_id) {
            Ordering::Equal => {
                // If object_ids are equal, compare type_id
                match self.type_id.cmp(&other.type_id) {
                    Ordering::Equal => {
                        // If type_ids are equal, compare offset
                        self.offset.cmp(&other.offset)
                    }
                    ordering => ordering,
                }
            }
            ordering => ordering,
        }
    }
}

impl PartialEq for BtrfsKey {
    fn eq(&self, other: &Self) -> bool {
        self.object_id == other.object_id
            && self.type_id == other.type_id
            && self.offset == other.offset
    }
}
impl Eq for BtrfsKey {}

impl PartialOrd for BtrfsItems {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BtrfsItems {
    fn cmp(&self, other: &Self) -> Ordering {
        // BtrfsItems are compared solely by their keys
        self.key.cmp(&other.key)
    }
}

impl PartialEq for BtrfsItems {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for BtrfsItems {}
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
