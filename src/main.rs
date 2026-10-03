// Include the CPU module.
mod cpu;

// Start the emulator executable.
fn main() {
    // Create a CPU with all registers and pointers set to zero.
    let _cpu = cpu::Cpu::default();

    // Print the emulator's name and target architecture.
    println!("arm-emu: AArch64 emulator");
}
