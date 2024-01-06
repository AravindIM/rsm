pub enum Exception {
    PageFault {
        pub message: String,
        pub mem_addr: u32,
        pub page_num: u32,
        pub cpu_mode: u32,
    },
    IllegalInstruction {
        pub message: String,
        pub mem_addr: u32,
        pub page_num: u32,
        pub cpu_mode: u32,
    },
    IllegalMemory {
        pub message: String,
        pub mem_addr: u32,
        pub page_num: u32,
        pub cpu_mode: u32,
    },
    Arithmetic {
        pub message: String,
        pub mem_addr: u32,
        pub page_num: u32,
        pub cpu_mode: u32,
    },
}
