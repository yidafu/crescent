use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};

use crate::compiler::ast::expression::Expression;

use super::linked_list::{Link, LinkedList};

const MAXARG_sBx: u64 = 0;
const OP_JMP: u64 = 0;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Any {}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    constants: HashMap<Any, u64>,
    used_regs: u64,
    max_regs: u64,

    scope_level: u64,
    local_variables: LinkedList<LocalVariableInfo>,
    local_names: HashMap<String, Link<LocalVariableInfo>>,

    parent: Option<Rc<FunctionInfo>>,
    upvalues: HashMap<String, UpvalueInfo>,

    breaks: Vec<Option<Vec<u64>>>,

    instructions: Vec<u32>,
}

impl FunctionInfo {
    pub fn index_of_constant(&mut self, key: Any) -> u64 {
        if self.constants.get(&key).is_some() {
            let idx = self.constants.get(&key).unwrap();
            idx.clone()
        } else {
            let idx = u64::try_from(self.constants.len()).unwrap();
            self.constants.insert(key, idx);
            idx
        }
        // self.constants.get(&key).unwrap();
    }

    pub fn alloc_register(&mut self) -> u64 {
        self.max_regs += 1;
        if self.used_regs > 255 {
            panic!("Function or expresion needs too many registers.");
        }

        if self.used_regs > self.max_regs {
            self.max_regs = self.used_regs;
        }

        self.used_regs - 1
    }

    pub fn alloc_registers(&mut self, n: u64) -> u64 {
        for _ in 0..n {
            self.alloc_register();
        }

        self.used_regs - n
    }

    pub fn free_register(&mut self) {
        self.used_regs -= 1;
    }

    pub fn free_registers(&mut self, n: u64) {
        for _ in 0..n {
            self.free_register();
        }
    }

    pub fn enter_scope(&mut self, breakable: bool) {
        self.scope_level += 1;

        if breakable {
            self.breaks.push(Some(Vec::new()))
        } else {
            self.breaks.push(None)
        }
    }

    pub fn exit_scope(&mut self) {
        let idx = self.breaks.len() - 1;

        let a = self.get_jump_arg_a();

        let pedding_break_jumps = self.breaks.get(idx).unwrap().as_ref().unwrap();

        for pc in pedding_break_jumps {
            let s_bx = self.pc() - pc.clone();

            let i = (s_bx + MAXARG_sBx) << 14 | a << 6 | OP_JMP;
            self.instructions[pc as usize] = i;
        }

        self.scope_level -= 1;
        self.local_names
            .clone()
            .into_iter()
            .for_each(|(name, var_info)| self.remove_local_variable(var_info));
    }

    pub fn pc(&self) -> u64 {
        todo!()
    }

    pub fn get_jump_arg_a(&mut self) -> u64 {
        todo!()
    }

    pub fn add_breack_jump(&mut self, pc: u64) {
        let mut i = self.scope_level;
        while i >= 0 {
            let break_vec = self.breaks.get(i as usize);

            if break_vec.is_some() && break_vec.unwrap().is_some() {
                let mut break_list = break_vec.unwrap().as_ref().unwrap().clone();
                break_list.push(pc);
                self.breaks.insert(i as usize, Some(break_list));
                return;
            }
        }
        panic!("<break> as line ? not inside a loop");
    }

    pub fn add_local_variable(&mut self, name: String) -> u64 {
        let local_variable_info = LocalVariableInfo::new(
            name.to_string(),
            self.scope_level,
            self.alloc_register(),
            false,
        );
        let slot = local_variable_info.slot;
        // let new_var = Rc::new(RefCell::new());
        self.local_variables.push(local_variable_info);

        self.local_names
            .insert(name, self.local_variables.head.take());

        // local_variable_info.slot
        slot
    }

    pub fn remove_local_variable(&mut self, mut local_variable: Link<LocalVariableInfo>) {
        self.free_register();

        let pre_node = LinkedList::next_node(local_variable.take());

        let var_info = LinkedList::peek_node_value(local_variable.clone()).unwrap();
        if pre_node.is_none() {
            self.local_names.remove(&var_info.name);
        } else {
            let prev_var_info = LinkedList::peek_node_value(local_variable).unwrap();
            if prev_var_info.scope_level == var_info.scope_level {
                self.remove_local_variable(pre_node);
            } else {
                let name = var_info.name;
                self.local_names.insert(name, pre_node);
            }
        }
    }

    pub fn get_lacal_variable_by_name(&mut self, name: String) -> Option<LocalVariableInfo> {
        let local_var = self.local_names.get(&name).unwrap().clone().unwrap();

        let local_var_value = LinkedList::peek_node_value(Some(local_var));

        local_var_value
    }

    pub fn slot_of_local_variable(&mut self, name: String) -> u64 {
        let var_name = self.get_lacal_variable_by_name(name);
        if var_name.is_some() {
            let var_info = var_name.unwrap();
            // local_node.slot
            var_info.slot
        } else {
            0
        }
    }

    pub fn index_of_upvalue(&mut self, name: String) -> u64 {
        let upvalue = self.upvalues.get(&name);
        if (upvalue.is_some()) {
            return upvalue.unwrap().index;
        } else if self.parent.is_some() {
            let local_var = Rc::try_unwrap(self.parent.clone().unwrap())
                .ok()
                .unwrap()
                .get_lacal_variable_by_name(name.clone());
            if local_var.is_some() {
                let local_var_value = local_var.unwrap();
                let idx = self.upvalues.len() as u64;
                self.upvalues.insert(
                    name,
                    UpvalueInfo {
                        local_variable_slot: local_var_value.slot,
                        upvale_index: 0,
                        index: idx,
                    },
                );
                return idx;
            } else {
                let uv_indx = Rc::try_unwrap(self.parent.clone().unwrap())
                    .ok()
                    .unwrap()
                    .index_of_upvalue(name.clone());
                if uv_indx >= 0 {
                    let idx = self.upvalues.len() as u64;
                    self.upvalues.insert(
                        name,
                        UpvalueInfo {
                            local_variable_slot: 0,
                            upvale_index: uv_indx,
                            index: idx,
                        },
                    );
                    return idx;
                } else {
                    return 0;
                }
            }
        }
        0
    }
}

#[derive(Debug, Clone)]
struct LocalVariableInfo {
    name: String,
    scope_level: u64,
    slot: u64,
    captured: bool,
}

impl LocalVariableInfo {
    pub fn new(name: String, scope_level: u64, slot: u64, captured: bool) -> LocalVariableInfo {
        LocalVariableInfo {
            name,
            scope_level,
            slot,
            captured,
        }
    }
}

#[derive(Debug, Clone)]
struct UpvalueInfo {
    local_variable_slot: u64,
    upvale_index: u64,
    index: u64,
}
