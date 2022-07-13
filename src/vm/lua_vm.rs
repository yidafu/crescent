use super::{
    binary_chunk::{Prototype, Upvalue, LocalVariable},
    instruction::{Instruction, InstructionOperation},
    lua_state::{LuaApi, LuaState, LuaVm},
    op_code::OpCodeEnum,
};

pub fn load_main(prototype: Prototype) {
    let n_regs = prototype.max_statck_size as i32;
    let mut state = LuaState::new((n_regs as usize) + 8, prototype);

    state.set_top( n_regs);
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
        abs_line_list:Vec::new(),
        local_variable: vec![LocalVariable {
            var_name: "a".to_string(),
            start_pc: 2,
            end_pc: 3,
        }],
        upvalue_names: vec!["_ENV".to_string()],
    };
    load_main(proto);
}
