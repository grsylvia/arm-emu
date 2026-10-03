// Initialize every field to zero by default.
#[derive(Default)]
// Store the CPU's general-purpose registers, stack pointer, and program counter.
pub struct Cpu {
    // Store the 64-bit registers X0 through X30.
    pub reg: [u64; 31],
    // Store the address of the current stack top.
    pub sp: u64,
    // Store the address of the instruction to execute.
    pub pc: u64,
}

// Include this module only when compiling tests.
#[cfg(test)]
// Group the CPU tests.
mod tests {
    // Bring the CPU type into scope.
    use super::Cpu;

    // Register this function as a test.
    #[test]
    // Check that the default CPU state is zero.
    fn default_cpu_starts_at_zero() {
        // Create a CPU with default field values.
        let cpu = Cpu::default();

        // Check that all 31 general-purpose registers are zero.
        assert_eq!(cpu.registers, [0; 31]);
        // Check that the stack pointer is zero.
        assert_eq!(cpu.stack_pointer, 0);
        // Check that the program counter is zero.
        assert_eq!(cpu.program_counter, 0);
    }
}
