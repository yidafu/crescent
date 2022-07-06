use super::{
    binary_chunk::Prototype,
    instruction::{Instruction, InstructionOperation},
    lua_state::{LuaApi, LuaState, LuaVm},
    op_code::OpCodeEnum,
};

pub fn load_main(prototype: Prototype) {
    let n_regs = prototype.max_statck_size as usize;
    let mut state = LuaState::new(n_regs + 8, prototype);

    state.set_top(n_regs.try_into().unwrap());
    loop {
        let pc = state.get_pc();
        let instruction: Instruction = state.fetch();
        let op_code = OpCodeEnum::try_from(instruction.op_code()).unwrap();
        match op_code {
            OpCodeEnum::OpReturn => break,
            code => instruction.execute(&state),
        }
    }
}
