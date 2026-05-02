use std::collections::HashMap;

use crate::{modules::{error::ForeignFnError, template::render_fn}, value::Value};

mod database;
mod template;
mod error;

type ForeignFn = fn(Vec<Value>) -> Result<Value, ForeignFnError>;

#[derive(Debug,Clone,Copy)]
struct FunctionContext {}

#[derive(Debug,Clone,Copy)]
struct ForeignFnEntry {
    idx: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionRegistry {
    func: HashMap<String, ForeignFnEntry>,
    idx_map: HashMap<u32, ForeignFn>,
    next_idx: u32,
    ctx: FunctionContext,
}

impl FunctionRegistry {
    pub fn new() -> Self {
         Self {
            func: HashMap::new(),
            idx_map: HashMap::new(),
            next_idx: 0,
            ctx: FunctionContext {},
        }
    }

    pub fn register(&mut self, name: String, function: ForeignFn) -> u32 {
        let idx = self.next_idx;
        self.next_idx += 1;

        let entry = ForeignFnEntry { idx };

        self.func.insert(name, entry);
        self.idx_map.insert(idx, function);

        idx
    }
    pub fn resolve(&self, idx: u32, args: Vec<Value>) -> Result<Value, ForeignFnError> {
        match self.idx_map.get(&idx) {
            Some(func) => func(args),
            None => panic!("Function index {} not found", idx),
        }
    }
    pub fn is_have_function(&self, name: &String) -> bool {
        self.func.contains_key(name)
    }
    pub fn get_idx_of_function(&self, name: &String) -> u32 {
        match self.func.get(name) {
            Some(entry) => entry.idx,
            None => panic!("Function '{}' not found", name),
        }
    }

    pub fn name_to_idx_map(&self) -> HashMap<String, u32> {
        self.func
            .iter()
            .map(|(name, entry)| (name.clone(), entry.idx))
            .collect()
    }
}


impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_default_foreign_function() -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();

    registry.register("render".to_string(), render_fn);

    registry
}




