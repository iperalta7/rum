#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    CMov,
    SegLoad,
    SegStore,
    Add,
    Mul,
    Div,
    BNand,
    Halt,
    MapSeg,
    UnmapSeg,
    Output,
    Input,
    LoadProg,
    LoadVal,
}


use core::panic;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::state::UniversalMachine;
type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn run(state: &mut UniversalMachine, instr: Vec<u32>){
    state.mapped_memory.push(instr);
    //let mut count = 0;
    loop {
        //count+=1;
        let instruction = state.mapped_memory.get(0).unwrap().get(state.program_counter).unwrap();
        state.program_counter += 1;
        disassemble(*instruction, state)
    }
}

pub fn disassemble(inst: Umi, state: &mut UniversalMachine) {
    match op(inst) {
        Some(Opcode::CMov) => {
            state.cmov(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::SegLoad) => {
            state.load(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::SegStore) => {
            state.store(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::Add) => {
            state.add(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::Mul) => {
            state.multiply(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::Div) => {
            state.division(get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::BNand) => {
            state.nand( get(&RA, inst), get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::Halt) => {
            //eprintln!("{}", count);
            state.halt()
        }
        Some(Opcode::MapSeg) => {
            state.map_seg(get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::UnmapSeg) => {
            state.unmap_seg(get(&RC, inst))
        }
        Some(Opcode::Output) => {
            state.output(get(&RC, inst))
        }
        Some(Opcode::Input) => {
            state.input(get(&RC, inst))
        }
        Some(Opcode::LoadProg) => {
            state.load_prog(get(&RB, inst), get(&RC, inst))
        }
        Some(Opcode::LoadVal) => {
            state.load_value(get(&RL, inst), get(&VL, inst))
        }
        None => panic!("Invalid Opcode")
    }
}