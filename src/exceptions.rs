use crate::errors::{ArithmeticError, DiskError, MemoryError, RegisterError};

pub enum MachineException {
    PageFault { page: i32 },
    IllegalInstruction { cause: IllegalInstructionException },
    IllegalMemory { cause: MemoryError, address: i32 },
    Arithmetic { cause: ArithmeticError },
}

pub enum IllegalInstructionException {
    NullInstruction,
    Malformed,
    Invalid,
    Unavailable,
    Privileged,
    InvalidMemoryDeref,
    WrongOperand,
    IncorrectLogical,
    StackOperationRegisterNotFound,
    InvalidInterruptNumber,
    InterruptInKernel,
    RegisterException { exception: RegisterError },
    DiskException { exception: DiskError },
}
