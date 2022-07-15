use std::io::Read;

use super::binary_chunk::{
    AbsoluteLine, LocalVariable, Prototype, Upvalue, Value,
    INSTRUCTION_SIZE, LUAC_DATA, LUAC_FORMAT, LUAC_INT, LUAC_NUM, LUAC_VERSION, LUA_INTEGER_SIZE,
    LUA_NUMBER_SIZE, LUA_SIGNATURE, TAG_FALSE, TAG_FLOAT, TAG_INTEGER, TAG_LONG_STRING, TAG_NIL,
    TAG_SHORT_STRING, TAG_TRUE,
};

pub type Unsigned = u64;

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

    fn read_unsigned(&mut self, mut limit: usize) -> usize {
        // let mut limit = 0xFFFFFFFF; // ~(size_t)0
        let mut x = 0 as usize;
        let mut b;
        limit >>= 7;
        loop {
            b = usize::try_from(self.read_byte()).unwrap();
            if x >= limit {
                panic!("integer overflow");
            }

            // 0x7f === 0b0111_0000
            x = (x << 7) | (b & 0b0111_1111);
            // 0x80 == 0b1000_0000
            // equal to b >= 128
            if (b & 0b1000_0000) != 0 {
                break;
            }
        }
        x
    }

    fn read_int(&mut self) -> i32 {
        self.read_unsigned(std::i32::MAX as usize) as i32
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

    fn read_size(&mut self) -> usize {
        self.read_unsigned(0xFFFFFFFF)
    }

    fn read_string(&mut self) -> String {
        // let a = self.read_u32();
        let size = self.read_size() as usize;
        if size == 0 {
            String::new()
        } else {
            let bytes = self.read_bytes(size - 1);

            String::from_utf8(bytes).unwrap()
        }
    }

    pub fn check_header(&mut self) {
        assert_eq!(
            self.read_bytes(4).as_slice(),
            LUA_SIGNATURE,
            "Not a precompiled chunk!"
        );
        assert_eq!(self.read_byte(), LUAC_VERSION, "Version mismatch!");
        assert_eq!(self.read_byte(), LUAC_FORMAT, "Format mismatch!");
        assert_eq!(self.read_bytes(6).as_slice(), LUAC_DATA, "corrupted!");
        // NOTE: lua 5.4 source code not check CINT_SIZE and CSIZET_SIEZE
        // assert_eq!(self.read_byte(), CINT_SIZE, "Int size mismatch!");
        // assert_eq!(self.read_byte(), CSIZET_SIEZE, "size_t size mismatch!");
        assert_eq!(
            self.read_byte(),
            INSTRUCTION_SIZE,
            "Instruction size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            LUA_INTEGER_SIZE,
            "Lua Integer size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            LUA_NUMBER_SIZE,
            "Lua Number size mismatch!"
        );
        assert_eq!(self.read_integer(), LUAC_INT, "endianness mismatch!");
        assert_eq!(self.read_number(), LUAC_NUM, "float format mismatch!");
    }

    pub fn read_function_prototype(&mut self, parent_source: String) -> Option<Prototype> {
        let mut source = self.read_string();
        if source == "" {
            source = parent_source;
        }

        Some(Prototype {
            source: source.clone(),
            line_defined: self.read_int(),
            last_line_defined: self.read_int(),
            num_params: self.read_byte(),
            is_vararg: self.read_byte(),
            max_statck_size: self.read_byte(),
            code: self.read_code(),
            constants: self.read_constants(),
            upvalues: self.read_upvalues(),
            prototypes: self.read_function_prototypes(source),
            line_info: self.read_line_info(),
            abs_line_list: self.read_absolute_list(),
            local_variable: self.read_local_variables(),
            upvalue_names: self.read_upvalue_names(),
        })
    }

    pub fn read_code(&mut self) -> Vec<u32> {
        let mut codes = Vec::new();
        let code_len = self.read_int();
        for _ in 0..code_len {
            codes.push(self.read_u32())
        }
        codes
    }

    pub fn read_constants(&mut self) -> Vec<Value> {
        let mut constants = Vec::new();
        let const_len = self.read_int();
        for _ in 0..const_len {
            constants.push(self.read_constant());
        }
        constants
    }

    pub fn read_constant(&mut self) -> Value {
        match self.read_byte() {
            TAG_NIL => Value::Nil,
            TAG_FALSE => Value::Boolean(self.read_byte() != 0),
            TAG_TRUE => Value::Boolean(self.read_byte() != 0),
            TAG_INTEGER => Value::Integer(self.read_integer()),
            TAG_FLOAT => Value::Number(self.read_number()),
            TAG_SHORT_STRING => Value::String(self.read_string()),
            TAG_LONG_STRING => Value::String(self.read_string()),
            v_tag => panic!("unknown value type: {}", v_tag),
        }
    }

    pub fn read_upvalues(&mut self) -> Vec<Upvalue> {
        let mut upvalues = Vec::new();
        let upvalue_len = self.read_int();
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
        let proto_len = self.read_int();
        for _ in 0..proto_len {
            prototypes.push(self.read_function_prototype(parent_source.clone()).unwrap());
        }

        Some(prototypes)
    }
    pub fn read_line_info(&mut self) -> Vec<u8> {
        let mut line_infos = Vec::new();
        let line_infos_len = self.read_int();
        for _ in 0..line_infos_len {
            line_infos.push(self.read_byte());
        }

        line_infos
    }

    pub fn read_absolute_list(&mut self) -> Vec<AbsoluteLine> {
        let mut abs_line_list = Vec::new();
        let abs_line_len = self.read_int();
        for _ in 0..abs_line_len {
            abs_line_list.push(AbsoluteLine {
                pc: self.read_u32(),
                line: self.read_u32(),
            })
        }

        abs_line_list
    }

    pub fn read_local_variables(&mut self) -> Vec<LocalVariable> {
        let mut local_variables = Vec::new();
        let local_variables_len = self.read_int();
        for _ in 0..local_variables_len {
            local_variables.push(LocalVariable {
                var_name: self.read_string(),
                start_pc: self.read_int(),
                end_pc: self.read_int(),
            })
        }
        local_variables
    }
    pub fn read_upvalue_names(&mut self) -> Vec<String> {
        let mut upvalue_names = Vec::new();
        let upvalue_names_len = self.read_int();
        for _ in 0..upvalue_names_len {
            upvalue_names.push(self.read_string())
        }
        upvalue_names
    }
}

#[test]
fn test_declare_a_variable() {
    let p = vec![
        0x1B, 0x4C, 0x75, 0x61, 0x54, 0x00, 0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A, 0x04, 0x08, 0x08,
        0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77,
        0x40, 0x01, 0x87, 0x40, 0x61, 0x2E, 0x6C, 0x75, 0x61, 0x80, 0x80, 0x00, 0x01, 0x02, 0x83,
        0x51, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0xC6, 0x00, 0x01, 0x01, 0x80, 0x81, 0x01,
        0x00, 0x00, 0x80, 0x83, 0x01, 0x00, 0x00, 0x80, 0x81, 0x82, 0x61, 0x82, 0x83, 0x81, 0x85,
        0x5F, 0x45, 0x4E, 0x56,
    ];

    let mut reader = LuaChunkReader::new(p);
    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string()).unwrap();
    println!("{:#?}", proto);
}

#[test]
fn test_hello_word_program() {
    let hello_word_program = vec![
        0x1B, 0x4C, 0x75, 0x61, // magic number ==> .Lua
        0x54, // lua version
        0x00, // format
        0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A, // 1993 CR NL SUB NL
        0x04, // 0x08, 0x04, // cint
        0x08, // size_t
        0x08, // instruction
        0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // lua int 0x7856
        0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77, 0x40, // lua number 370.5
        0x01, // not vararg
        0x92, 0x40, 0x2E, 0x2F, // ==> 5.4
        // helle_word.lua
        0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x5F, 0x77, 0x6F, 0x72, 0x64, 0x2E, 0x6C, 0x75, 0x61,
        0x80, // line define == 1
        0x80, // last line defined
        0x00, // num params
        0x01, // is_vararg
        0x02, // max stack size
        0x85, // count of codes
        // 5 instructions
        0x51, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0x83, 0x80, 0x00, 0x00, 0x44, 0x00, 0x02,
        0x01, 0x46, 0x00, 0x01, 0x01, 0x82, // count of constant
        0x04, 0x86, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x04, // print
        0x8C, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x4C, 0x75, 0x61, 0x21,
        0x81, // hello, Loa
        0x01, // count of upvalue
        0x00, 0x00, // instack and index
        0x80, // function prototype count
        0x85, // count of line info
        0x01, 0x00, 0x00, 0x00, 0x00, // list of line info
        0x80, // absolute line count
        0x80, // local variable count
        0x81, // upvalue name count
        0x85, 0x5F, 0x45, 0x4E, 0x56, // _ENV
    ];

    let mut reader = LuaChunkReader::new(hello_word_program);
    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string()).unwrap();
    println!("{:?}", proto);
    assert_eq!(proto.source, "@./hello_word.lua");
    assert_eq!(proto.is_vararg, 1);
    match proto.constants.get(0).unwrap() {
        Value::String(str) => assert_eq!(str, "print"),
        _ => panic!("not print string"),
    }
    assert_eq!(proto.upvalue_names[0], "_ENV");
}

#[test]
fn test_echo_function() {
    let echo_function_program = vec![
        0x1B, 0x4C, 0x75, 0x61, 0x54, 0x00, 0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A, 0x04, 0x08, 0x08,
        0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77,
        0x40, 0x01, 0x90, 0x40, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x5F, 0x77, 0x6F, 0x72, 0x64, 0x2E,
        0x6C, 0x75, 0x61, // main function
        0x80, 0x80, 0x00, 0x01, 0x02, 0x87, // instructions
        0x51, 0x00, 0x00, 0x00, 0x4F, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00,
        0x00, 0x83, 0x80, 0x00, 0x00, 0x44, 0x00, 0x02, 0x01, 0x46, 0x00, 0x01, 0x01, 0x82, 0x04,
        0x85, 0x65, 0x63, 0x68, 0x6F, 0x04, 0x84, 0x4C, 0x75, 0x61, 0x81, 0x01, 0x00, 0x00,
        // function prototype start
        0x81, 0x80, 0x81, 0x83, 0x01, 0x00, 0x03, 0x86, // constants
        0x8B, 0x00, 0x00, 0x00, 0x03, 0x81, 0x00, 0x00, 0x22, 0x01, 0x02, 0x00, 0x2E, 0x01, 0x00,
        0x06, 0xC4, 0x00, 0x02, 0x01, 0xC7, 0x00, 0x01, 0x00, 0x82, 0x04, // type string
        0x86, // string len 6
        0x70, 0x72, 0x69, 0x6E, 0x74, // print
        0x04, 0x88, // string 4
        0x48, 0x65, 0x6C, 0x6C, 0x6F, // hello
        0x2C, 0x20, 0x81, // upvalue count 2
        0x00, 0x00, 0x00,
        // ======== inner prototype end ========
        // function prototype 0
        0x80, 0x86, // line info
        0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80, // abs line info
        0x81, // local variable 1
        0x85, 0x6E, 0x61, 0x6D, 0x65, // name
        0x80, // start pc
        0x86, // end pc
        0x81, // up names 1
        0x85, // string len 5
        0x5F, 0x45, 0x4E, 0x56,
        // ======== inner prototype end ========
        // rest prototype defintion
        0x87, 0x01, 0x02, 0xFE, 0x04, 0x00, 0x00, 0x00, 0x80, 0x80, 0x81, 0x85, 0x5F, 0x45, 0x4E,
        0x56,
    ];
    let mut reader = LuaChunkReader::new(echo_function_program);
    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string()).unwrap();

    println!("{:#?}", proto);
}

#[test]
fn test_read_size() {
    let mut reader = LuaChunkReader::new(vec![
        0x1B, 0x4C, 0x75, 0x61, 0x54, 0x00, 0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A, 0x04, 0x08, 0x08,
        0x78, 0x56, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x77,
        0x40, 0x01, 0x87, 0x40, 0x61, 0x2E, 0x6C, 0x75, 0x61, 0x80, 0x80, 0x00, 0x01, 0x03, 0x87,
        0x51, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x80, 0x81, 0x80, 0x00, 0x80, 0x08, 0x01, 0x00,
        0x00, 0x22, 0x01, 0x00, 0x01, 0x2E, 0x00, 0x01, 0x06, 0xC6, 0x01, 0x01, 0x01, 0x80, 0x81,
        0x01, 0x00, 0x00, 0x80, 0x87, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x83, 0x82,
        0x61, 0x84, 0x87, 0x82, 0x62, 0x84, 0x87, 0x82, 0x63, 0x84, 0x87, 0x81, 0x85, 0x5F, 0x45,
        0x4E, 0x56,
    ]);

    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string()).unwrap();

    println!("{:#?}", proto);
}

#[test]
fn dump_chunk_file() {
    let file = std::fs::File::open("/Users/yidafu/github/Language/crescent/float.luac").unwrap();
    let mut buf = Vec::new();
    std::io::BufReader::new(file).read_to_end(&mut buf);

    let mut reader = LuaChunkReader::new(buf);

    reader.check_header();
    reader.read_byte();
    let proto = reader.read_function_prototype("".to_string()).unwrap();
    println!("{:#?}", proto);
}
