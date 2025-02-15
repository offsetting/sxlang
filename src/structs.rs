use binrw::*;

#[derive(Debug)]
#[binrw(little)]
pub struct DebugInfo {
    pub file_name_offset: u32,
    pub line_offset: u32,
}

#[derive(Debug)]
#[binrw(little)]
pub struct Variable {
    pub name_offset: u32,
    pub variable_type: u32,
    pub size: i16,
}

#[derive(Debug)]
#[binrw(little)]
pub struct StaticVariable {
    pub name_offset: u32,
    pub unk: [u16; 4],
}

#[derive(Debug)]
#[binrw(little)]
pub struct Unk {
    pub unk: [u32; 3],
    pub unk2: [u16; 2],
}

#[binrw(little)]
#[derive(Debug)]
pub struct Function {
    pub name_offset: u32,
    pub name_hash: u32,
    pub address: u32,
    pub stack_delta: i32,
}

#[binrw(little)]
#[derive(Debug)]
pub struct VariableLookup {
    pub hash: u32,
    pub variable_index: u32,
}

#[binrw(little)]
#[derive(Debug)]
pub struct LabelLookup {
    pub hash: u32,
    pub name_offset: u32,
    pub program_counter: u32,
}

#[binrw(little)]
#[derive(Debug)]
pub struct SwitchTable {
    pub value: u32,
    pub program_counter: u32,
}
