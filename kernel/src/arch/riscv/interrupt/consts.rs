pub const Syscall: usize = 8;
pub const InstructionPageFault: usize = 12;
pub const LoadPageFault: usize = 13;
pub const StorePageFault: usize = 15;

// highest bit set
pub const IrqMin: usize = usize::MAX / 2;
pub const IrqMax: usize = usize::MAX;

pub const Timer: usize = usize::MAX / 2 + 1 + 5;
pub const SupervisorExternal: usize = usize::MAX / 2 + 1 + 8;

pub fn is_page_fault(trap: usize) -> bool {
    trap == InstructionPageFault || trap == LoadPageFault || trap == StorePageFault
}
