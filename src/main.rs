mod opcodes;
mod structs;

use std::io::Seek;

use crate::opcodes::*;
use binrw::*;

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
    /*
    SlangInstruction opcodes[opcode_count];
    SlangDebugInfo debug_infos[debug_info_count];
    SlangVariable variables[variable_count];
    SlangStaticVariable statics[static_count];
    SlangUnk unks[unk_count];
    SlangFunction functions[function_count];
    SlangVariableLookup variable_lookups[variable_lookup_count];
    SlangLabelLookup label_lookups[label_lookup_count];
    SlangSwitchTable switch_tables[switch_table_count];
    u32 strings[string_count];
    */
}

fn main() {
    let mut script_file = std::fs::File::open("../genericanimatoranim.sx").unwrap();
    let script = Header::read_le(&mut script_file).unwrap();
    dbg!(script.instrctions);
    dbg!(script_file.stream_position().unwrap());
}
