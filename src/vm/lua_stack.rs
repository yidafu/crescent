use super::binary_chunk::Value;

#[derive(Debug)]
pub struct LuaStack {
    pub slots: Vec<Value>,
    pub top: usize,
}

impl LuaStack {
    pub fn new(size: usize) -> LuaStack {
        return LuaStack {
            slots: Vec::with_capacity(size),
            top: 0,
        };
    }

    pub fn check(&mut self, n: usize) {
        let free = self.slots.len() - self.top;
        for _ in free..n {
            self.slots.push(Value::Nil);
        }
    }

    pub fn push(&mut self, val: Value) {
        self.slots.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Value {
        // let slots = self.slots;
        // let top_val = &slots[self.top];
        // slots[self.top] = Value::Nil;
        self.top -= 1;
        self.slots.pop().unwrap()
    }

    pub fn abs_index(&self, index: i32) -> usize {
        if index >= 0 {
            index as usize
        } else {
            ((self.top as i32) + index + 1) as usize
        }
    }

    pub fn is_valid(&self, index: i32) -> bool {
        let abs_idx = self.abs_index(index);

        0 < abs_idx && abs_idx <= self.top
    }

    pub fn get(&mut self, index: i32) -> Value {
        let abs_idx = self.abs_index(index);
        let val = &self.slots[abs_idx];
        val.clone()
    }

    pub fn set(&mut self, index: i32, val: Value) {
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
    stack.push(Value::Boolean(true));
    assert_eq!(stack.top, 1);
    stack.push(Value::Integer(1));

    stack.push(Value::Number(2.0));
    stack.push(Value::String("string".to_string()));
    stack.push(Value::Nil);

    let pop_value = stack.pop();
    assert_eq!(pop_value, Value::Nil);

    assert_eq!(stack.get(2), Value::Number(2.0));
    stack.set(2, Value::Number(3.0));
    assert_eq!(stack.get(2), Value::Number(3.0));

    stack.check(7);
    println!("{:#?}", stack);
}
