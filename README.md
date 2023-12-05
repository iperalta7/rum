# Assignment 4: Rust Universal Machine (RUM)
## Isaiah Peralta
## Due Date: 12/1/2023

## Acknowledgements 
- Giving Acknowledments to TA Help hours for support in: 
    - starting a sound structure of UniversalMachine 
    - connecting disassembled instructions to my um 
    - Helping fix my div, mult, add, etc. The operations. I kept getting a 'divisor of zero' panic. 
- Credits to Professor Daniels for rumdump lab as this made me understand it a lot more and borrowed methods from that lab (load, opcode enums). 

## Implementation 
Goal of this assingment is to understand virtual machine code by programming the Rust Universal Machine (RUM), which a is a simple virtual machine implemented in Rust.

I believe my implementation correctly: 
- Uses bitshift to parse opcodes, registers, and values
- represents the state of Universal Machine
- follows the implementation for each seperate instruction

## Architecture 
As far as departures from design, just probable modified some names for modules and methods I had initially. 

### Modules: 
- main.rs 
This is where the program starts and just runs. Simply used to for argument parsing and getting the instructions to interact with the state of the machine. 

- load.rs
Boilerplate pretty much. Used for getting the binary of a um program. Ouputs a Vec<u32>/instructions which is read in by my `UniversalMachine`` struct. (taken from rumdump lab)

- rumdis.rs
rumdis.rs is responsible for most of the opcode parsing from an instruction. `Run()` interacts with `UniversalMachine` to load up the initial instruction and then continue to parse the rest. `disassemble()` uses fields and some bitshifting helper methods to to gather the correct registers and pass them to corresponding instruction methods (in `state.rs``).

- state.rs (invariants described)
Module is used to hold the representation for my UniversalMachine which has the following data representation: 
- Registers
The machine is equipped with eight general-purpose registers, represented as a vector of u32.
- Mapped Memory (Segmented)
 Memory in the RUM is organized into segments and offsets, forming a two-dimensional vector of u32.
The first index represents a memory segment, while the second index represents an offset within that segment.
- Unmapped memory (free segment)
The UM has a vector called `unmapped_memory`, where each index represents an unmapped memory segment.
 Used to allocate and deallocate memory as needed during the execution of programs.
-  Opcode Instructions
The UM has set of 14 opcode instructions that is disassembled in rumdis.rs and then calling the appropriate method to that instruction in the struct:
- Conditional Move
- Load
- Store
- Add
- Multiply
- Division
- Bitwise NAND
- Halt
- Map Segment
- Unmap Segment
- Output
- Input
- Load Program
- Load Value


## Benchmark 50 Million instructions
My Laptop CPU: Intel(R) Core(TM) i5-1035G1 CPU @ 1.00GHz, 1190 Mhz, 4 Core(s), 8 Logical Processor(s)

Based on the 2113497561 instructions ran from sandmark.

Time(s): 
| attempt | time (seconds) | 
| --------| ---- |
| 1 | 25.677 |
| 2 | 32.663 | 
| 3 | 27.135 |  
| 4 | 28.095 |
| 5 | 25.664 |

On average of 5 runs, rum takes 27.8468s for 2113497561 instructions. 
Therefore, I do 27.8468 / ( 2113497561/50000000 ) to get the time for 50 million instructions: approxiamtely 0.6588s. 

## Time
- I spent around 8-10 hours analyzing and then probably another 6-8 designing UM. 
- After analysis, I spent a decent amount of nights trying to program so probably 15-20 hours 