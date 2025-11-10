#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Constant {
    Number(i32),
    String(String),
}

