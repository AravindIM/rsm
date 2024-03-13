pub const RSM_WORD_SIZE: usize = 16;
pub const RSM_MEMORY_NUM_PAGES: usize = 128;
pub const RSM_PAGE_SIZE: usize = 512;
pub const RSM_MEMORY_NUM_WORDS: usize = RSM_MEMORY_NUM_PAGES * RSM_PAGE_SIZE;
pub const RSM_DISK_BLOCKSIZE: usize = RSM_PAGE_SIZE;
pub const RSM_REG_SIZE: usize = RSM_WORD_SIZE;
pub const RSM_NUM_REG: usize = 33;

pub const RSM_INSTRUCTION_SIZE: usize = 2;
pub const REG_PORT_LOW: usize = 20;
pub const REG_PORT_HIGH: usize = 23;
pub const REG_KERN_LOW: usize = 27;
pub const REG_KERN_HIGH: usize = 32;
