#[derive(Debug)]
pub struct BinaryChunk {
    header: Header,
    sized_upvalues: i32,
    main_func: Box<Prototype>,
}

pub const LUA_SIGNATURE: [u8; 4] = [0x1B, 0x4C, 0x75, 0x61]; // => 0x1B4c7561=> ".Lua"
pub const LUAC_VERSION: u8 = 0x54;
pub const LUAC_FORMAT: u8 = 0;
pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A];
pub const CINT_SIZE: u8 = 4;
pub const CSIZET_SIEZE: u8 = 8;
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

#[derive(Debug)]
pub struct Header {
    signature: [u8; 4],
    version: u8,
    format: u8,
    luac_data: [u8; 6],
    cint_size: u8,
    sizet_size: u8,
    instruction_size: u8,
    lua_number_size: u8,
    lua_integer_size: u8,
    luac_int: i64,
    luac_num: f64,
}

#[derive(Debug)]
pub struct AbsoluteLine {
    pub pc: u32,
    pub line: u32,
}

#[derive(Debug)]
pub struct Prototype {
    pub source: String,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_statck_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Value>,
    pub upvalues: Vec<Upvalue>,
    pub prototypes: Option<Vec<Prototype>>,
    pub line_info: Vec<u8>,
    pub abs_line_list: Vec<AbsoluteLine>,
    pub local_variable: Vec<LocalVariable>,
    pub upvalue_names: Vec<String>,
}

pub const TAG_NIL: u8 = 0x00;
pub const TAG_BOOLEAN: u8 = 0x01;
pub const TAG_NUMBER: u8 = 0x03;
pub const TAG_INTEGER: u8 = 0x13;
pub const TAG_SHORT_STRING: u8 = 0x04;
pub const TAG_LONG_STRING: u8 = 0x14;

#[derive(Debug)]
pub struct Upvalue {
    pub instack: u8,
    pub index: u8,
}

#[derive(Debug)]
pub struct LocalVariable {
    pub var_name: String,
    pub start_pc: i32,
    pub end_pc: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl TryInto<i64> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Integer(val) => Ok(val),
            _ => Err("must be Value:Integer(i64)"),
        }
    }
}

#[test]
fn fn_test() {
    let a = [1, 2, 5];
}
