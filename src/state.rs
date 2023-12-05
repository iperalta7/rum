use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Clone)]

/// Representation of Universal Machine
/// Here are several invariants:
///
/// ## Registers
/// The machine is equipped with eight general-purpose registers, represented as a vector.
/// Each register is accessed using indices from 0 to 7, with each index corresponding to an individual register.
///
/// ## Segmented Memory
/// Memory in the RUM is organized into segments and offsets, forming a two-dimensional vector.
/// The first index represents a memory segment, while the second index represents an offset within that segment.
///
/// ## Opcode Instructions
/// The UM a set of 14 opcode instructions:
/// - Conditional Move
/// - Load
/// - Store
/// - Add
/// - Multiply
/// - Division
/// - Bitwise NAND
/// - Halt
/// - Map Segment
/// - Unmap Segment
/// - Output
/// - Input
/// - Load Program
/// - Load Value
///
/// Each instruction is implemented as a dedicated function
///
/// ## Free Segments
/// The UM has a vector called `unmapped_memory`, where each index represents an unmapped memory segment.
/// Used to allocate and deallocate memory as needed during the execution of programs.
pub struct UniversalMachine {
    registers: [u32; 8], // Eight general-purpose registers holding one word each
    pub mapped_memory: Vec<Vec<u32>>,
    unmapped_memory: Vec<u32>, 
    pub program_counter: usize,
}

impl UniversalMachine{

    /// Creates a new instance of the UniversalMachine with default values.
    ///
    /// Each register is initialized with the minimum value of u32, and the memory is empty.
    pub fn new() -> Self {
        Self {
            registers: [u32::MIN; 8],
            mapped_memory: Vec::new(),
            unmapped_memory: Vec::new(),
            program_counter: 0,
        }
    }

    /// Conditional move instruction.
    ///
    /// Moves the value from register `b` to register `a` if the value in register `c` is not zero.
    pub fn cmov(&mut self, a: u32, b: u32, c: u32) {
        if self.registers[c as usize] == 0 {
            return;
        }

        self.registers[a as usize] = self.registers[b as usize];
    }

    /// Load instruction.
    ///
    /// Loads the value from the memory segment specified by registers `b` and `c`
    /// into register `a`.
    pub fn load(&mut self, a: u32, b: u32, c: u32) {
        let reg_b = self.registers[b as usize] as usize;
        let reg_c = self.registers[c as usize] as usize;

        self.registers[a as usize] = self.mapped_memory[reg_b][reg_c];
    }

    /// Store instruction.
    ///
    /// Stores the value from register `c` into the memory segment specified by registers `a` and `b`.
    pub fn store(&mut self, a: u32, b: u32, c: u32) {
        let reg_a = self.registers[a as usize] as usize;
        let reg_b = self.registers[b as usize] as usize;
        self.mapped_memory[reg_a][reg_b] = self.registers[c as usize];
    }

    /// Add instruction.
    ///
    /// Adds the values in registers `b` and `c` and stores the result in register `a`.
    pub fn add(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = self.registers[b as usize].wrapping_add(self.registers[c as usize]);
    }

    /// Multiply instruction.
    ///
    /// Multiplies the values in registers `b` and `c` and stores the result in register `a`.
    pub fn multiply(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = self.registers[b as usize].wrapping_mul(self.registers[c as usize]);
    }

    /// Division instruction.
    ///
    /// Divides the value in register `b` by the value in register `c`
    /// and stores the result in register `a`.
    ///
    /// # Panics
    ///
    /// Panics if attempting to divide by zero.
    pub fn division(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = self.registers[b as usize].wrapping_div(self.registers[c as usize]);
    }

    /// NAND instruction.
    ///
    /// Computes the bitwise NAND of the values in registers `b` and `c`
    /// and stores the result in register `a`.
    pub fn nand(&mut self, a: u32, b: u32, c: u32) {
        self.registers[a as usize] = !(self.registers[b as usize] & self.registers[c as usize]);
    }

    /// Halt instruction.
    ///
    /// Exits the program.
    pub fn halt(&mut self) {
        std::process::exit(0);
    }

    /// Map Segment instruction.
    ///
    /// Creates a new memory segment with a capacity specified by the value in register `c`.
    /// The index of the newly mapped segment is stored in register `b`.
    pub fn map_seg(&mut self, b: u32, c: u32) {
        let new_seg = vec![0_u32; self.registers[c as usize] as usize];

        let new_seg_idx = self.unmapped_memory.pop().unwrap_or_else(|| {
            self.mapped_memory.push(new_seg.clone());
            self.mapped_memory.len() as u32 - 1
        });

        self.registers[b as usize] = new_seg_idx;

        self.mapped_memory[new_seg_idx as usize] = new_seg;
    }

    /// Unmap Segment instruction.
    ///
    /// Frees the memory of the memory segment specified by the value in register `c`.
    pub fn unmap_seg(&mut self, c: u32) {
        let free_seg = self.registers[c as usize];
        self.mapped_memory[free_seg as usize].clear();
        self.unmapped_memory.push(free_seg);
    }

    /// Output instruction.
    ///
    /// Prints the ASCII character corresponding to the value in register `c`.
    pub fn output(&mut self, c: u32) {
        let r = u8::try_from(self.registers[c as usize]).unwrap();
        print!("{}", r as char);
    }

    /// Input instruction.
    ///
    /// Reads a character from standard input and stores its ASCII value in register `c`.
    ///
    /// If there is no input available, the register is set to the maximum value of u32.
    pub fn input(&mut self, c: u32) {
        match stdin().bytes().next() {
            Some(input) => self.registers[c as usize] = input.unwrap() as u32,
            None => self.registers[c as usize] = !0_u32,
        }
    }

    /// Load Program instruction.
    ///
    /// Loads the memory segment specified by the value in register `b` into the program memory.
    ///
    /// If the location is 0, sets the program counter to the value in register `c`.
    pub fn load_prog(&mut self, b: u32, c: u32){
        let location = self.registers[b as usize] as usize;
        if location == 0 {
            self.program_counter = self.registers[c as usize] as usize;
            return
        }
        self.mapped_memory[0] = self.mapped_memory[location].clone();
        self.program_counter = self.registers[c as usize] as usize;
    }

    /// Load Value instruction.
    ///
    /// Loads the given value at the given register 'a'.
    pub fn load_value(&mut self, a: u32, val: u32){
        self.registers[a as usize] = val
    }


}
