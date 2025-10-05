use super::tree::Tree;
use super::value::Value;

pub const NREG: usize = 16;

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

    pub fn set(&mut self, idx: usize, value: Value) {
        self.regs[idx] = value;
    }

    pub fn set_new_table(&mut self, idx: usize) {
        self.regs[idx] = Value::Table(Box::new(Tree::new()));
    }

    pub fn set_attr_table(&mut self, idx: usize, key: String, value: Value) {
        if let Value::Table(box_to_tree) = &mut self.regs[idx] {
            // box_to_tree.childrens.entry(key).or_insert(Tree::new_with_value(Some(value)));
            box_to_tree.insert(vec![&key], value).ok();
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
