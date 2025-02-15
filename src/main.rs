#![feature(seek_stream_len)]

mod opcodes;
mod structs;

use std::io::{Read, Seek};

use crate::opcodes::*;
use binrw::*;
use structs::*;

#[binrw::parser(reader: r, endian)]
fn read_instructions(opcode_count: u32) -> binrw::BinResult<Vec<Instruction>> {
    let instructions_end_offset = r.stream_position().unwrap() + opcode_count as u64 * 2;
    let mut instructions = Vec::with_capacity(opcode_count as usize);
    while r.stream_position().unwrap() < instructions_end_offset {
        let raw = u16::read_options(r, endian, ()).unwrap();
        let opcode = Opcode::from((raw & 0xFF) as u8);
        let data = (raw >> 8) as u8;
        let instruction = match opcode {
            Opcode::PushVar => {
                let var = u16::read_options(r, endian, ()).unwrap();
                Instruction::PushVar(var)
            }
            Opcode::PushMultipleVars => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::PushMultipleVars(data, index)
            }
            Opcode::PushVarIndexed => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::PushVarIndexed(index)
            }
            Opcode::PushInt => {
                let value = i32::read_options(r, endian, ()).unwrap();
                Instruction::PushInt(value)
            }
            Opcode::PushFloat => {
                let value = f32::read_options(r, endian, ()).unwrap();
                Instruction::PushFloat(value)
            }
            Opcode::PushByte => Instruction::PushByte(data),
            Opcode::PushByteAsFloat => Instruction::PushByteAsFloat(data),
            Opcode::PushString => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::PushString(index)
            }
            Opcode::Pop => Instruction::Pop,
            Opcode::PopAndStore => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::PopAndStore(index)
            }
            Opcode::PopAndStoreMultiple => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::PopAndStoreMultiple(data, index)
            }
            Opcode::Store => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::Store(index)
            }
            Opcode::StoreIndexed => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::StoreIndexed(index)
            }
            Opcode::Dup => Instruction::Dup,
            Opcode::Swap => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::Swap(index)
            }
            Opcode::PushOwner => Instruction::PushOwner,
            Opcode::LessThan => Instruction::LessThan,
            Opcode::LessThanEq => Instruction::LessThanEq,
            Opcode::GtrThan => Instruction::GtrThan,
            Opcode::GtrThanEq => Instruction::GtrThanEq,
            Opcode::Equal => Instruction::Equal,
            Opcode::NotEqual => Instruction::NotEqual,
            Opcode::Add => Instruction::Add,
            Opcode::Sub => Instruction::Sub,
            Opcode::Mult => Instruction::Mult,
            Opcode::Div => Instruction::Div,
            Opcode::Mod => Instruction::Mod,
            Opcode::Negate => Instruction::Negate,
            Opcode::Abs => Instruction::Abs,
            Opcode::ShiftL => Instruction::ShiftL,
            Opcode::ShiftR => Instruction::ShiftR,
            Opcode::LogicalAnd => Instruction::LogicalAnd,
            Opcode::LogicalOr => Instruction::LogicalOr,
            Opcode::LogicalNot => Instruction::LogicalNot,
            Opcode::BitwiseAnd => Instruction::BitwiseAnd,
            Opcode::BitwiseOr => Instruction::BitwiseOr,
            Opcode::BitwiseXor => Instruction::BitwiseXor,
            Opcode::BitwiseNot => Instruction::BitwiseNot,
            Opcode::BranchTrue => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::BranchTrue(target_address)
            }
            Opcode::BranchFalse => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::BranchFalse(target_address)
            }
            Opcode::BranchTrueNoPop => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::BranchTrueNoPop(target_address)
            }
            Opcode::BranchFalseNoPop => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::BranchFalseNoPop(target_address)
            }
            Opcode::Jump => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::Jump(target_address)
            }
            Opcode::Call => {
                let target_address = u16::read_options(r, endian, ()).unwrap();
                Instruction::Call(target_address)
            }
            Opcode::SysCall => {
                let target = u16::read_options(r, endian, ()).unwrap();
                Instruction::SysCall(target)
            }
            Opcode::Return => Instruction::Return,
            Opcode::Switch => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::Switch(index)
            }
            Opcode::Unk2 => {
                let index = u16::read_options(r, endian, ()).unwrap();
                Instruction::Unk2(index)
            }
            Opcode::Unk3 => Instruction::Unk3,
            Opcode::Unk4 => Instruction::Unk4,
            Opcode::End => Instruction::End,
            Opcode::Sleep => Instruction::Sleep,
            Opcode::Print => Instruction::Print,
            Opcode::Target => Instruction::Target,
            Opcode::Nop => Instruction::Nop,
        };

        instructions.push(instruction);
    }
    Ok(instructions)
}

#[binrw(little)]
#[derive(Debug)]
struct Header {
    pub magic: u32,
    pub version: u32,
    pub status: u32,
    pub size: u32,
    pub compiled_file_name_offset: u32,
    pub source_file_name_offset: u32,
    pub opcode_count: u32,
    pub debug_info_count: u32,
    pub variable_count: u32,
    pub static_count: u32,
    pub unk_count: u32,
    pub function_count: u32,
    pub string_count: u32,
    pub variable_lookup_count: u32,
    pub label_lookup_count: u32,
    pub switch_table_count: u32,
    pub opcodes_offset: u32,
    pub debug_infos_offset: u32,
    pub variables_offset: u32,
    pub statics_offset: u32,
    pub unks_offset: u32,
    pub functions_offset: u32,
    pub variable_lookup_offset: u32,
    pub label_lookup_offset: u32,
    pub switch_tables_offset: u32,
    pub string_table_offset: u32,

    #[br(parse_with = read_instructions, args(opcode_count))]
    pub instrctions: Vec<Instruction>,

    #[br(count = debug_info_count)]
    pub debug_infos: Vec<DebugInfo>,

    #[br(count = variable_count)]
    pub variables: Vec<Variable>,

    #[br(count = static_count)]
    pub static_variables: Vec<StaticVariable>,

    #[br(count = unk_count)]
    pub unks: Vec<Unk>,

    #[br(count = function_count)]
    pub functions: Vec<Function>,

    #[br(count = variable_lookup_count)]
    pub variable_lookups: Vec<VariableLookup>,

    #[br(count = label_lookup_count)]
    pub label_lookups: Vec<LabelLookup>,

    #[br(count = switch_table_count)]
    pub switch_tables: Vec<SwitchTable>,

    #[br(count = string_count)]
    pub string_offsets: Vec<u32>,
}

const NULL_BYTE: u8 = b'\0';

pub(crate) fn read_cstr(input: &[u8]) -> String {
    let mut output = String::new();

    for c in input {
        if c == &NULL_BYTE {
            return output;
        }
        output.push(*c as char);
    }

    output
}

fn main() {
    let mut script_file = std::fs::File::open("../cloth.sx").unwrap();
    let script = Header::read_le(&mut script_file).unwrap();
    dbg!(script);
    dbg!(script_file.stream_position().unwrap());
}
