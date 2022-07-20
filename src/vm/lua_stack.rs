use super::lua_value::LuaValue;

#[derive(Debug)]
pub struct LuaStack {
    pub slots: Vec<LuaValue>,
    pub top: usize,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        let mut stack = LuaStack {
            slots: Vec::with_capacity(size),
            top: 0,
        };
        for _ in 0..size {
            stack.slots.push(LuaValue::Nil);
        }
        stack
    }

    pub fn check(&mut self, n: usize) {
        let free = self.slots.len() - self.top;
        for _ in free..n {
            self.slots.push(LuaValue::Nil);
        }
    }

    pub fn push(&mut self, val: LuaValue) {
        assert!(self.top < self.slots.len(), "stack overflow");
        self.slots[self.top] = val;
        self.top += 1;
    }

    pub fn pop(&mut self) -> LuaValue {
        assert!(self.top > 0, "stack underflow");
        self.top -= 1;
        let val = self.slots.get_mut(self.top).unwrap().clone();
        self.slots[self.top] = LuaValue::Nil;
        val
    }

    pub fn abs_index(&self, index: i32) -> usize {
        if index >= 0 {
            index as usize
        } else {
            ((self.top as i32) + index) as usize
        }
    }

    pub fn is_valid(&self, index: i32) -> bool {
        let abs_idx = self.abs_index(index);

        0 < abs_idx && abs_idx <= self.top
    }

    pub fn get(&mut self, index: i32) -> LuaValue {
        let abs_idx = self.abs_index(index);
        let val = &self.slots[abs_idx];
        val.clone()
    }

    pub fn set(&mut self, index: i32, val: LuaValue) {
        let abs_idx = self.abs_index(index);
        self.slots[abs_idx] = val;
    }

    pub fn reverse(&mut self, mut from: i32, mut to: i32) {
        while from < to {
            let from_val = self.get(from);
            let to_val = self.get(to);
            self.set(from, to_val);
            self.set(to, from_val);
            from += 1;
            to -= 1;
        }
    }
}

#[test]
fn test_lua_stack() {
    let mut stack = LuaStack::new(7);
    stack.push(LuaValue::Boolean(true));
    assert_eq!(stack.top, 1);
    stack.push(LuaValue::Integer(1));

    stack.push(LuaValue::Number(2.0));
    stack.push(LuaValue::String("string".to_string()));
    stack.push(LuaValue::Nil);

    let pop_value = stack.pop();
    assert_eq!(pop_value, LuaValue::Nil);

    assert_eq!(stack.get(2), LuaValue::Number(2.0));
    stack.set(2, LuaValue::Number(3.0));
    assert_eq!(stack.get(2), LuaValue::Number(3.0));

    stack.check(7);
    println!("{:#?}", stack);
}
