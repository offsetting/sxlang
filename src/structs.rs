use binrw::*;

#[binrw(little)]
pub struct DebugInfo {
    pub file_name_offset: u32,
    pub line_offset: u32,
}

#[binrw(little)]
pub struct Variable {
    pub name_offset: u32,
    pub variable_type: u32,
    pub size: i16,
}

#[binrw(little)]
pub struct StaticVariable {
    pub name_offset: u32,
    pub unk: [u16; 4],
}

#[binrw(little)]
pub struct Unk {
    pub unk: [u32; 3],
    pub unk2: [u16; 2],
}

#[binrw(little)]
pub struct Function {
    pub name_offset: u32,
    pub name_hash: u32,
    pub address: u32,
    pub stack_delta: i32,
}

#[binrw(little)]
struct VariableLookup {
    pub hash: u32,
    pub variable_index: u32,
}

#[binrw(little)]
struct LabelLookup {
    pub hash: u32,
    pub name_offset: u32,
    pub program_counter: u32,
}

#[binrw(little)]
struct SwitchTable {
    pub value: u32,
    pub program_counter: u32,
}
