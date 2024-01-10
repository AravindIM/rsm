pub enum Exception {
    PageFault {
        message: String,
        page_num: u32,
        cpu_mode: u32,
    },
    IllegalInstruction {
        message: String,
        cpu_mode: u32,
    },
    IllegalMemory {
        message: String,
        mem_addr: u32,
        cpu_mode: u32,
    },
    Arithmetic {
        message: String,
        cpu_mode: u32,
    },
}
