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
    pub constants: Vec<LuaValue>,
    pub upvalues: Vec<Upvalue>,
    pub prototypes: Option<Vec<Prototype>>,
    pub line_info: Vec<u8>,
    pub abs_line_list: Vec<AbsoluteLine>,
    pub local_variable: Vec<LocalVariable>,
    pub upvalue_names: Vec<String>,
}

pub const TAG_NIL: u8 = 0b0;
pub const TAG_FALSE: u8 = 0b1;
pub const TAG_TRUE: u8 = 0b1_0001;
pub const TAG_INTEGER: u8 = 0b11;
pub const TAG_FLOAT: u8 = 0b1_0011;
pub const TAG_SHORT_STRING: u8 = 0b100;
pub const TAG_LONG_STRING: u8 = 0b1_100;

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
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl TryInto<i64> for LuaValue {
    type Error = &'static str;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            LuaValue::Integer(val) => Ok(val),
            LuaValue::Number(v) => Ok(v.round() as i64),
            LuaValue::String(v) => {
                let res = v.parse::<i64>();
                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    let f_res = v.parse::<f64>();
                    if f_res.is_ok() {
                        Ok(f_res.unwrap().round() as i64)
                    } else {
                        Err("could not convert string to int")
                    }
                }
            }
            _ => Err("Lua Value must be Integer/Number/String"),
        }
    }
}

impl TryInto<f64> for LuaValue {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            LuaValue::Integer(v) => Ok(v as f64),
            LuaValue::Number(v) => Ok(v),
            LuaValue::String(v) => {
                let res = v.parse::<f64>();
                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    Err("could not convert String to Number")
                }
            }
            _ => Err("Lua Value must be Integer/Number/String"),
        }
    }
}

#[test]
fn fn_test() {
    let a = [1, 2, 5];
}
