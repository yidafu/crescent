use super::{
    binary_chunk::{LocalVariable, Prototype, Upvalue, Value},
    instruction::{Instruction, InstructionOperation},
    lua_state::{LuaApi, LuaState, LuaVm},
    op_code::OpCodeEnum,
};

pub fn load_main(prototype: Prototype) {
    let n_regs = prototype.max_statck_size as i32;
    let mut state = LuaState::new((n_regs as usize) + 8, prototype);

    state.set_top(n_regs);
    loop {
        let pc = state.get_pc();
        let instruction: Instruction = state.fetch();
        let op_code = OpCodeEnum::try_from(instruction.op_code()).unwrap();
        match op_code {
            OpCodeEnum::OpReturn => break,
            _code => {
                instruction.execute(&mut state);
                println!("pc: {:?}, op name: {:?}", pc + 1, instruction.op_name());
            }
        }
    }
}

#[test]
fn test_declare_a_variable() {
    let proto = Prototype {
        source: "@a.lua".to_string(),
        line_defined: 0,
        last_line_defined: 0,
        num_params: 0,
        is_vararg: 1,
        max_statck_size: 2,
        code: [81, 8, 16842950].to_vec(),
        constants: Vec::new(),
        upvalues: vec![Upvalue {
            instack: 1,
            index: 0,
        }],
        prototypes: Some(Vec::new()),
        line_info: [1, 0, 0].to_vec(),
        abs_line_list: Vec::new(),
        local_variable: vec![LocalVariable {
            var_name: "a".to_string(),
            start_pc: 2,
            end_pc: 3,
        }],
        upvalue_names: vec!["_ENV".to_string()],
    };
    load_main(proto);
}

#[test]
fn test_add_2_integer_program() {
    let proto = Prototype {
        source: "@a.lua".to_string(),
        line_defined: 0,
        last_line_defined: 0,
        num_params: 0,
        is_vararg: 1,
        max_statck_size: 3,
        code: vec![
            81, 2147483649, 2147516545, 264, 16777506, 100728878, 16843206,
        ],
        constants: vec![],
        upvalues: vec![Upvalue {
            instack: 1,
            index: 0,
        }],
        prototypes: Some(vec![]),
        line_info: vec![1, 0, 0, 0, 0, 0, 0],
        abs_line_list: vec![],
        local_variable: vec![
            LocalVariable {
                var_name: "a".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
            LocalVariable {
                var_name: "b".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
            LocalVariable {
                var_name: "c".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
        ],
        upvalue_names: vec!["_ENV".to_string()],
    };

    load_main(proto);
    // 1       [1]     VARARGPREP      0
    // 2       [1]     LOADI           0 1
    // 3       [1]     LOADI           1 2
    // 4       [1]     LOADNIL         2 0     ; 1 out
    // 5       [1]     ADD             2 0 1
    // 6       [1]     MMBIN           0 1 6   ; __add
    // 7       [1]     RETURN          3 1 1   ; 0 out
}

#[test]
fn test_add_2_float_program() {
    let proto = Prototype {
        source: "@./a.lua".to_string(),
        line_defined: 0,
        last_line_defined: 0,
        num_params: 0,
        is_vararg: 1,
        max_statck_size: 3,
        code: vec![81, 3, 32899, 264, 16777506, 100728878, 16843206],
        constants: vec![Value::Number(2.2), Value::Number(3.3)],
        upvalues: vec![Upvalue {
            instack: 1,
            index: 0,
        }],
        prototypes: Some(vec![]),
        line_info: vec![1, 0, 0, 0, 1, 0, 0],
        abs_line_list: vec![],
        local_variable: vec![
            LocalVariable {
                var_name: "a".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
            LocalVariable {
                var_name: "b".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
            LocalVariable {
                var_name: "c".to_string(),
                start_pc: 4,
                end_pc: 7,
            },
        ],
        upvalue_names: vec!["_ENV".to_string()],
    };
    // 1       [1]     VARARGPREP      0
    // 2       [1]     LOADK           0 0     ; 2.2
    // 3       [1]     LOADK           1 1     ; 3.3
    // 4       [1]     LOADNIL         2 0     ; 1 out
    // 5       [2]     ADD             2 0 1
    // 6       [2]     MMBIN           0 1 6   ; __add
    // 7       [2]     RETURN          3 1 1   ; 0 out
    load_main(proto);
}
