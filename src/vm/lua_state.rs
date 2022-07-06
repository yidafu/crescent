use crate::vm::binary_chunk::Value;

use super::{binary_chunk::Prototype, lua_stack::LuaStack};

#[derive(Debug)]
pub struct LuaState {
    stack: LuaStack,
    // prototype: &mut Prototype,
    pub pc: u32,
}

impl LuaState {
    pub fn new(stack_size: usize) -> LuaState {
        LuaState {
            stack: LuaStack::new(stack_size),
            // prototype,
            pc: 0,
        }
    }
    pub fn get_top(&self) -> usize {
      self.stack.top
    }

    pub fn abs_index(&mut self, index: i32) -> usize {
      self.stack.abs_index(index)
    }

    pub fn check_stack(&mut self, n: usize) -> bool {
      self.stack.check(n);
      true
    }

    pub fn pop(&mut self, n: usize) {
      for _ in 0..n {
        self.stack.pop();
      }
    }

    pub fn copy(&mut self, from_idex: i32, to_index: i32) {
      let val = self.stack.get(from_idex);
      self.stack.set(to_index, val);
    }

    pub fn push_value(&mut self, index: i32) {
      let val = self.stack.get(index);
      self.stack.push(val);
    }

    pub fn replace(&mut self, index: i32) {
      let val = self.stack.pop();
      self.stack.set(index, val);
    }

    pub fn inster(&mut self, index: i32) {
      self.rotate(index, -1);
      self.pop(1);
    }

    pub fn rotate(&mut self, index: i32, n: i32) {
      let top_index = (self.get_top() - 1) as i32;
      let abs_index = self.abs_index(index) as i32;

      let m: i32 = if n > 0 {
        top_index - n
      } else {
        abs_index - n - 1
      };

      self.stack.reverse(abs_index, m);
      self.stack.reverse(m+1, top_index);
      self.stack.reverse(abs_index, top_index);
    }

    pub fn set_top(&mut self, index: i32) {
      let new_top = self.abs_index(index);
      assert!(new_top >= 0, "stack underflow");

      let n = self.get_top() - new_top;
      if n > 0 {
        for _ in 0..n {
            self.stack.pop();
        } 
      } else if n < 0 {
        for _ in 0..n {
          self.stack.push(Value::Nil)
        }
      }
    }
}
