use super::{
    binary_chunk::Prototype, instruction::Instruction, lua_stack::LuaStack, lua_value::LuaValue,
};

#[derive(Debug)]
pub struct LuaState {
    pub stack: LuaStack,
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

pub trait LuaVm: LuaApi {
    fn get_pc(&self) -> u32;
    fn add_pc(&mut self, n: i32);
    fn fetch(&mut self) -> Instruction;
    fn get_const(&mut self, idx: usize);
    fn get_pk(&mut self, rk: i32);

    fn arith(
        &mut self,
        i_func: Option<fn(a: i64, a: i64) -> i64>,
        f_func: Option<fn(a: f64, b: f64) -> f64>,
    );
}

impl LuaVm for LuaState {
    fn get_pc(&self) -> u32 {
        self.pc
    }

    fn add_pc(&mut self, n: i32) {
        self.pc = ((self.pc as i32) + n) as u32;
    }

    fn fetch(&mut self) -> Instruction {
        let instr = self.prototype.code[self.pc as usize];
        self.pc += 1;
        return instr;
    }

    fn get_const(&mut self, idx: usize) {
        let constant = self.prototype.constants.get(idx).unwrap();
        self.stack.push(constant.clone());
    }

    fn get_pk(&mut self, rk: i32) {
        if rk > 0xff {
            self.get_const((rk as usize) & 0xff);
        } else {
            self.push_value(rk);
        }
    }

    fn arith(
        &mut self,
        i_func: Option<fn(a: i64, a: i64) -> i64>,
        f_func: Option<fn(a: f64, b: f64) -> f64>,
    ) {
        let val_c = self.stack.pop();
        let val_b = self.stack.pop();

        // TODO: remove clone method
        let b = val_b.clone().try_into();
        let c = val_c.clone().try_into();
        if f_func.is_none() {
            if i_func.is_some() {
                let d = i_func.unwrap()(b.unwrap(), c.unwrap());
                self.push_integer(d);
            }
        } else {
            if i_func.is_some() {
                let d = i_func.unwrap()(b.unwrap(), c.unwrap());
                self.push_integer(d);
                return;
            }
            let f_b = val_b.try_into();
            let f_c = val_c.try_into();
            let d = f_func.unwrap()(f_b.unwrap(), f_c.unwrap());
            self.push_number(d);
        }
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
    fn insert(&mut self, index: i32);
    fn rotate(&mut self, index: i32, n: i32);
    fn set_top(&mut self, index: i32);

    fn push_nil(&mut self);
    fn push_integer(&mut self, val: i64);
    fn push_boolean(&mut self, val: bool);
    fn push_string(&mut self, val: String);
    fn push_number(&mut self, val: f64);

    fn is_number(&mut self, idx: usize) -> bool;
    fn to_numberx(&mut self, idx: usize) -> Option<f64>;

    fn is_integer(&mut self, idx: usize) -> bool;
    fn to_integer(&mut self, idx: usize) -> Option<i64>;

    fn is_string(&mut self, idx: i32) -> bool;
    fn to_string(&mut self, idx: i32) -> Option<String>;

    fn len(&mut self, idx: i32);
    fn concat(&mut self, idx: usize);

    fn compare(&mut self, idx1: i32, idex2: i32, op: CampareOperator) -> bool;
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

    fn insert(&mut self, index: i32) {
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

        let n: i32 = (self.get_top() as i32) - (new_top as i32);
        if n > 0 {
            for _ in 0..n {
                self.stack.pop();
            }
        } else if n < 0 {
            for _ in n..0 {
                self.stack.push(LuaValue::Nil)
            }
        }
    }

    fn push_nil(&mut self) {
        self.stack.push(LuaValue::Nil);
    }

    fn push_integer(&mut self, val: i64) {
        self.stack.push(LuaValue::Integer(val));
    }

    fn push_boolean(&mut self, val: bool) {
        self.stack.push(LuaValue::Boolean(val));
    }

    fn push_string(&mut self, val: String) {
        self.stack.push(LuaValue::String(val));
    }

    fn push_number(&mut self, val: f64) {
        self.stack.push(LuaValue::Number(val));
    }

    fn is_number(&mut self, idx: usize) -> bool {
        self.to_numberx(idx).is_some()
    }

    fn to_numberx(&mut self, idx: usize) -> Option<f64> {
        let val = self.stack.get(idx.try_into().unwrap());
        match val {
            LuaValue::Integer(v) => Some(v as f64),
            LuaValue::Number(v) => Some(v),
            _ => None,
        }
    }

    fn is_integer(&mut self, idx: usize) -> bool {
        self.to_integer(idx).is_some()
    }

    fn to_integer(&mut self, idx: usize) -> Option<i64> {
        let val = self.stack.get(idx as i32);
        match val {
            LuaValue::Integer(v) => Some(v),
            _ => None,
        }
    }

    fn is_string(&mut self, idx: i32) -> bool {
        self.to_string(idx).is_some()
    }

    fn to_string(&mut self, idx: i32) -> Option<String> {
        let val = self.stack.get(idx);
        match val {
            LuaValue::String(s) => Some(s),
            _ => None,
        }
    }

    fn len(&mut self, idx: i32) {
        let val = self.stack.get(idx);
        match val {
            LuaValue::String(s) => self.push_integer(s.len() as i64),
            _ => panic!("Only String has length"),
        };
    }

    fn concat(&mut self, idx: usize) {
        if idx == 0 {
            self.stack.push(LuaValue::String("".to_string()));
        } else if idx >= 2 {
            if self.is_string(-1) && self.is_string(-2) {
                let s2 = self.to_string(-1).unwrap();
                let s1 = self.to_string(-2).unwrap();
                self.stack.pop();
                self.stack.pop();
                self.stack.push(LuaValue::String(s1 + &s2));
            } else {
                panic!("concat string error!");
            }
        }
        //  else {
        //     do nothine
        //  }
    }

    fn compare(&mut self, idx1: i32, idx2: i32, op: CampareOperator) -> bool {
        let a_val = self.stack.get(idx1);
        let b_val = self.stack.get(idx2);
        println!("compare a: {:?}, b: {:?}", a_val, b_val);
        match op {
            CampareOperator::Equal => a_val == b_val,
            CampareOperator::LessThen => a_val < b_val,
            CampareOperator::GreatThen => a_val > b_val,
        }
    }
}

pub enum CampareOperator {
    Equal,
    LessThen,
    GreatThen,
}
