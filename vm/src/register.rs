use crate::tree::Tree;

pub const NREG: usize = 16;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    String(String),
    Table(Box<Tree<Value>>),
    Thunk(u32),
    None
}

impl Clone for Value {

    fn clone(&self) -> Self {
        match self {
            Self::Int(arg0) => Self::Int(*arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Table(arg0) => Self::Table(arg0.clone()),
            Self::Thunk(arg0) => Self::Thunk(*arg0),
            Self::None => Self::None,
        }
    }
} 


#[derive(Debug)]
pub struct Register {
    regs: [Value; NREG],
}

impl Register {
    pub fn new() -> Self {
        Register {
            regs: [
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
            ],
        }
    }

    pub fn get(&self, idx: usize) -> Option<Value>{
        self.regs.get(idx).cloned()
    }

    pub fn set_int(&mut self, idx: usize, int: i64) {
        self.regs[idx] = Value::Int(int);
    }

    pub fn set_str(&mut self, idx: usize, string: String) {
        self.regs[idx] = Value::String(string);
    }

    pub fn set_new_table(&mut self, idx: usize) {
        self.regs[idx] = Value::Table(Box::new(Tree::new()));
    }

    pub fn set_attr_table(&mut self, idx: usize, key: String, value: Value) {
        if let Value::Table(box_to_tree) = &mut self.regs[idx] {
            box_to_tree.childrens.entry(key).or_insert(Tree::new_with_value(Some(value)));
        }
    }
    pub fn reset_all(&mut self) {
        self.regs = [
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
        ]
    }

}

impl Default for Register {

    fn default() -> Self {
        Self {
            regs: [
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::None,

            ]
        }
    }
}
