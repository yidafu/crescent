use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use super::binary_chunk::{
    LocalVariable, Prototype, Upvalue, Value, CINT_SIZE, CSIZET_SIEZE, INSTRUCTION_SIZE, LUAC_DATA,
    LUAC_FORMAT, LUAC_INT, LUAC_NUM, LUAC_VERSION, LUA_INTEGER_SIZE, LUA_NUMBER_SIZE,
    LUA_SIGNATURE, TAG_BOOLEAN, TAG_INTEGER, TAG_LONG_STRING, TAG_NIL, TAG_NUMBER,
    TAG_SHORT_STRING,
};

pub struct LuaChunkReader {
    pub buffer: Vec<u8>,
    pub debug_byte: u8,
    pub index: usize,
}

impl LuaChunkReader {
    pub fn new(buffer: Vec<u8>) -> LuaChunkReader {
        LuaChunkReader {
            buffer,
            index: 0,
            debug_byte: 0,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) {
        self.buffer = buf.to_vec();
        self.read_byte();
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.buffer[self.index];
        self.index += 1;
        self.debug_byte = byte;
        byte
    }

    fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        for _ in 0..n {
            bytes.push(self.read_byte());
        }

        bytes
    }

    fn read_u32(&mut self) -> u32 {
        let a1 = self.read_byte() as u32;
        let a2 = self.read_byte() as u32;
        let a3 = self.read_byte() as u32;
        let a4 = self.read_byte() as u32;
        a4 << 24 | a3 << 16 | a2 << 8 | a1
    }

    fn read_uint64(&mut self) -> u64 {
        let a1 = self.read_u32() as u64;
        let a2 = self.read_u32() as u64;
        a2 << 32 | a1
    }

    fn read_integer(&mut self) -> i64 {
        let integer = self.read_uint64();
        i64::try_from(integer).unwrap()
    }

    fn read_number(&mut self) -> f64 {
        f64::from_bits(self.read_uint64())
    }

    fn read_string(&mut self) -> String {
        // let a = self.read_u32();
        let mut size = self.read_byte() as usize;
        if size == 0 {
            return "".to_string();
        }

        if size == 0xFF {
            size = self.read_uint64() as usize;
        }

        let bytes = self.read_bytes(size - 1);

        String::from_utf8(bytes).unwrap()
    }

    pub fn check_header(&mut self) {
        if self.read_bytes(4).as_slice() != LUA_SIGNATURE {
            panic!("Not a precompiled chunk!");
        }

        if self.read_byte() != LUAC_VERSION {
            panic!("Version mismatch!");
        }

        if self.read_byte() != LUAC_FORMAT {
            panic!("Format mismatch!");
        }
        if self.read_bytes(6).as_slice() != LUAC_DATA {
            panic!("corrupted!");
        }
        // NOTE: lua source code not check CINT_SIZE and CSIZET_SIEZE
        if self.read_byte() != CINT_SIZE {
            panic!("Int size mismatch!");
        }
        if self.read_byte() != CSIZET_SIEZE {
            panic!("Size_t size mismatch!");
        }
        if self.read_byte() != INSTRUCTION_SIZE {
            panic!("Instruction size mismatch!");
        }
        if self.read_byte() != LUA_INTEGER_SIZE {
            panic!("Lua Integer size mismatch!");
        }
        if self.read_byte() != LUA_NUMBER_SIZE {
            panic!("Lua Number size mismatch!");
        }
        if self.read_integer() != LUAC_INT {
            panic!("endianness mismatch!");
        }
        if self.read_number() != LUAC_NUM {
            panic!("float format mismatch!");
        }
    }

    pub fn read_function_prototype(&mut self, parent_source: String) -> Option<Prototype> {
        let mut source = self.read_string();
        if source == "" {
            source = parent_source;
        }

        Some(Prototype {
            source: source.clone(),
            line_defined: self.read_u32(),
            last_line_defined: self.read_u32(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_statck_size: self.read_byte(),
            code: self.read_code(),
            constants: self.read_constants(),
            upvalues: self.read_upvalues(),
            prototypes: self.read_function_prototypes(source),
            line_info: self.read_line_info(),
            local_variable: self.read_local_variables(),
            upvalue_names: self.read_upvalue_names(),
        })
    }

    pub fn read_code(&mut self) -> Vec<u32> {
        let mut codes = Vec::new();
        let code_len = self.read_u32();
        for _ in 0..code_len {
            codes.push(self.read_u32())
        }
        codes
    }

    pub fn read_constants(&mut self) -> Vec<Value> {
        let mut constants = Vec::new();
        let const_len = self.read_u32();
        for _ in 0..const_len {
            constants.push(self.read_constant());
        }
        constants
    }

    pub fn read_constant(&mut self) -> Value {
        match self.read_byte() {
            TAG_NIL => Value::Nil,
            TAG_BOOLEAN => Value::Boolean(self.read_byte() != 0),
            TAG_INTEGER => Value::Integer(self.read_integer()),
            TAG_NUMBER => Value::Number(self.read_number()),
            TAG_SHORT_STRING => Value::String(self.read_string()),
            TAG_LONG_STRING => Value::String(self.read_string()),
            v_tag => panic!("unknown value type: {}", v_tag),
        }
    }

    pub fn read_upvalues(&mut self) -> Vec<Upvalue> {
        let mut upvalues = Vec::new();
        let upvalue_len = self.read_u32();
        for _ in 0..upvalue_len {
            upvalues.push(Upvalue {
                instack: self.read_byte(),
                index: self.read_byte(),
            });
        }

        upvalues
    }

    pub fn read_function_prototypes(&mut self, parent_source: String) -> Option<Vec<Prototype>> {
        let mut prototypes = Vec::new();
        let proto_len = self.read_u32();
        for _ in 0..proto_len {
            prototypes.push(self.read_function_prototype(parent_source.clone()).unwrap());
        }

        Some(prototypes)
    }
    pub fn read_line_info(&mut self) -> Vec<u32> {
        let mut line_infos = Vec::new();
        let line_infos_len = self.read_u32();
        for _ in 0..line_infos_len {
            line_infos.push(self.read_u32());
        }

        line_infos
    }
    pub fn read_local_variables(&mut self) -> Vec<LocalVariable> {
        let mut local_variables = Vec::new();
        let local_variables_len = self.read_u32();
        for _ in 0..local_variables_len {
            local_variables.push(LocalVariable {
                var_name: self.read_string(),
                start_pc: self.read_u32(),
                end_pc: self.read_u32(),
            })
        }
        local_variables
    }
    pub fn read_upvalue_names(&mut self) -> Vec<String> {
        let mut upvalue_names = Vec::new();
        let upvalue_names_len = self.read_u32();
        for _ in 0..upvalue_names_len {
            upvalue_names.push(self.read_string())
        }
        upvalue_names
    }
}

#[test]
fn test_hello_word_program() {
    let hello_word_program = vec![
        0x1B, 0x4C, 0x75, 0x61, // magic number ==> .Lua
        0x54, // lua version
        0x00, // format
        0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A, // 1993 CR NL SUB NL
        0x04,
        0x08,
        0x04, // cint
        0x08, // size_t
        0x08, // instruction
        0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // lua int 0x7856
        0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77, 0x40, // lua number 370.5
        0x01, // not vararg
        0x12, 0x40, 0x2E, 0x2F,
        // 0x92, 0x40, 0x2E, 0x2F, // ==> 5.4
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x5F, 0x77, 0x6F, 0x72, 0x64, 0x2E, 0x6C, 0x75, 0x61,  // helle_word.lua
        // 0x80,
        // 0x80, 0x00, 0x01, 0x02, 0x85, 0x51, 0x00, 0x00, 0x00, 0x0B, 0x00,
        // 0x00, 0x00, 0x83, 0x80, 0x00, 0x00, 0x44, 0x00, 0x02, 0x01, 0x46, 0x00, 0x01, 0x01, 0x82,
        // 0x04, 0x86, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x04, 0x8C, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C,
        // 0x20, 0x4C, 0x75, 0x61, 0x21, 0x81, 0x01, 0x00, 0x00, 0x80, 0x85, 0x01, 0x00, 0x00, 0x00,
        // 0x00, 0x80, 0x80, 0x81, 0x85, 0x5F, 0x45, 0x4E, 0x56,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x04,
0x00, 0x00, 0x00, 0x06, 0x00, 0x40, 0x00, 0x41, 0x40, 0x00, 0x00, 0x24, 0x40, 0x00, 0x01, 0x26,
0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x06, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x04, 0x0C,
0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x4C, 0x75, 0x61, 0x21, 0x01, 0x00, 0x00, 0x00, 0x01,
0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
0x00, 0x05, 0x5F, 0x45, 0x4E, 0x56
    ];

    let mut reader = LuaChunkReader::new(hello_word_program);
    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string());

    print!("{:#?}", proto);
}
