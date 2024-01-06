pub enum Exception {
    PageFault {
        message: String,
        mem_addr: u32,
        page_num: u32,
        cpu_mode: u32,
    },
    IllegalInstruction {
        message: String,
        mem_addr: u32,
        page_num: u32,
        cpu_mode: u32,
    },
    IllegalMemory {
        message: String,
        mem_addr: u32,
        page_num: u32,
        cpu_mode: u32,
    },
    Arithmetic {
        message: String,
        mem_addr: u32,
        page_num: u32,
        cpu_mode: u32,
    },
}
