pub enum RegisterError {
    Invalid,
    Privileged,
    IPAccessed,
}

pub enum DiskError {
    InvalidPage,
    InvalidBlock,
    WrongArguments,
}

pub enum MemoryError {
    PageFault,
    AccessViolation,
    OutsideLogicalSpace,
    IllegalAccess,
}

pub enum ArithmeticError {
    DivByZero,
    ModByZero,
}
