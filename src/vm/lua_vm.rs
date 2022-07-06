use super::{binary_chunk::Prototype, lua_state::LuaState};


pub fn load_main(prototype: &mut Prototype) {
  let n_regs = prototype.max_statck_size as usize;
  let mut state = LuaState::new(n_regs + 8);
  state.set_top(n_regs.try_into().unwrap());
  loop {
    let pc = state.pc;
  }
}