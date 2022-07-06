use crate::vm::binary_chunk::Value;

use super::{binary_chunk::Prototype, instruction::Instruction, lua_stack::LuaStack};

#[derive(Debug)]
pub struct LuaState {
    stack: LuaStack,
    pub prototype: Prototype,
    pub pc: u32,
}

impl LuaState {
    pub fn new(stack_size: usize, prototype: Prototype) -> LuaState {
        LuaState {
            stack: LuaStack::new(stack_size),
            prototype,
            pc: 0,
        }
    }
}

pub trait LuaVm {
    fn get_pc(&self) -> u32;
    fn add_pc(&mut self, n: u32);
    fn fetch(&mut self) -> Instruction;
    fn get_const(&self, idx: u32) -> u32;
    fn get_pk(&self, rk: u32);
}

impl LuaVm for LuaState {
    fn get_pc(&self) -> u32 {
        self.pc
    }

    fn add_pc(&mut self, n: u32) {
        self.pc += n;
    }

    fn fetch(&mut self) -> Instruction {
        let instr = self.prototype.code[self.pc as usize];
        self.pc += 1;
        return instr;
    }

    fn get_const(&self, idx: u32) -> u32 {
        todo!()
    }

    fn get_pk(&self, rk: u32) {
        todo!()
    }
}

pub trait LuaApi {
    fn get_top(&self) -> usize;
    fn abs_index(&mut self, index: i32) -> usize;
    fn check_stack(&mut self, n: usize) -> bool;
    fn pop(&mut self, n: usize);
    fn copy(&mut self, from_idex: i32, to_index: i32);
    fn push_value(&mut self, index: i32);
    fn replace(&mut self, index: i32);
    fn inster(&mut self, index: i32);
    fn rotate(&mut self, index: i32, n: i32);
    fn set_top(&mut self, index: i32);
}

impl LuaApi for LuaState {
    fn get_top(&self) -> usize {
        self.stack.top
    }

    fn abs_index(&mut self, index: i32) -> usize {
        self.stack.abs_index(index)
    }

    fn check_stack(&mut self, n: usize) -> bool {
        self.stack.check(n);
        true
    }

    fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.stack.pop();
        }
    }

    fn copy(&mut self, from_idex: i32, to_index: i32) {
        let val = self.stack.get(from_idex);
        self.stack.set(to_index, val);
    }

    fn push_value(&mut self, index: i32) {
        let val = self.stack.get(index);
        self.stack.push(val);
    }

    fn replace(&mut self, index: i32) {
        let val = self.stack.pop();
        self.stack.set(index, val);
    }

    fn inster(&mut self, index: i32) {
        self.rotate(index, -1);
        self.pop(1);
    }

    fn rotate(&mut self, index: i32, n: i32) {
        let top_index = (self.get_top() - 1) as i32;
        let abs_index = self.abs_index(index) as i32;

        let m: i32 = if n > 0 {
            top_index - n
        } else {
            abs_index - n - 1
        };

        self.stack.reverse(abs_index, m);
        self.stack.reverse(m + 1, top_index);
        self.stack.reverse(abs_index, top_index);
    }

    fn set_top(&mut self, index: i32) {
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
