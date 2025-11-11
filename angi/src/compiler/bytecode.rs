mod constant;
mod function;
mod thunk;
mod load_global;

pub use load_global::load_global;
use super::{ast::{Expr, Operator}, error::BytecodeGenerationError};
use constant::Constant;
use core::panic;
use function::Function;
use instructions::{MAGIC_NUMBER, METADATA_BYTES, OpCode, VERSION};
use std::collections::HashMap;
use thunk::Thunk;

#[derive(Default)]
pub struct BytecodeGen {
    pub constants: HashMap<Constant, usize>,
    pub thunks: Vec<Thunk>,
    pub functions: Vec<Function>,
    pub global_functions: HashMap<String, Function>,
    pub global_function_in_used: HashMap<String, usize>,
    pub ins_count: u32,
    pub register_in_used: [bool; 16],
    pub ins_code: Vec<u8>,
    context_var: HashMap<String, u8>,
}

impl BytecodeGen {

    pub fn new() -> Self {
        BytecodeGen {
            constants: HashMap::new(),
            thunks: vec![],
            functions: vec![],
            global_functions: HashMap::new(),
            global_function_in_used: HashMap::new(),
            ins_count: 0,
            register_in_used: [false; 16],
            ins_code: vec![],
            context_var: HashMap::new(),
        }
    }

    pub fn with_global_func(mut self, global_func: HashMap<String, Function>) -> Self {
        self.global_functions = global_func;
        self
    }

    pub fn get_binary(&mut self, expr: Expr) -> 
    Result<Vec<u8>, BytecodeGenerationError> 
    {
        let reg = self.visit_expr(&expr, false)?;

        self.free_register(reg as usize);
        self.emit_ins(OpCode::RETURN.encode(vec![reg as u32]));

        let _ = self.visit_remain_thunk();
        let _ = self.visit_function();

        let mut bytes = vec![];
        self.add_header_to_binary(&mut bytes);
        self.add_const_to_binary(&mut bytes);
        self.add_thunk_table_to_binary(&mut bytes);
        self.add_function_table_to_binary(&mut bytes);
        self.add_global_function_table_to_binary(&mut bytes);
        self.add_ins_code_to_binary(&mut bytes);
        self.add_footer_to_binary(&mut bytes);

        Ok(bytes)
    }

    pub fn visit_expr(&mut self, expr: &Expr, is_make_thunk: bool) -> Result<u8, BytecodeGenerationError>{
        match expr {
            Expr::Number(num) => {
                let idx_const = self.make_const(Constant::Number(*num));
                let reg_value = self
                    .get_register()
                    .expect("Error in get register: the value");
                self.emit_ins(OpCode::LOADCONST.encode(vec![
                    reg_value as u32,
                    idx_const.try_into().expect("Error when convert idx_const to u32"),
                ]));
                Ok(reg_value)
            }
            Expr::LiteralString(str) => {
                let idx_const = self.make_const(Constant::String(str.to_string()));
                let reg_value = self
                    .get_register()
                    .expect("Error in get register: the value");
                self.emit_ins(OpCode::LOADCONST.encode(vec![
                    reg_value as u32,
                    idx_const.try_into().expect("Error when convert idx_const to u32"),
                ]));
                Ok(reg_value)
            }
            Expr::Binary { op, lhs, rhs } => {
                let lhs_reg = self.visit_expr(lhs, false)?;

                let rhs_reg = self.visit_expr(rhs, false)?;

                let reg_value = self
                    .get_register()
                    .expect("Error in get register: the value");

                let opcode = match op {
                    Operator::Add => OpCode::ADD,
                    Operator::Sub => OpCode::SUB,
                    Operator::Div => OpCode::DIV,
                    Operator::Mul => OpCode::MUL,
                };

                self.emit_ins(opcode.encode(vec![
                    reg_value as u32,
                    lhs_reg as u32,
                    rhs_reg as u32,
                ]));
                self.free_register(reg_value as usize);
                self.free_register(lhs_reg as usize);
                self.free_register(rhs_reg as usize);
                Ok(reg_value)
            }
            Expr::Var(name) => {
                let reg_value = self.context_var.get(name);
                match reg_value {
                    Some(reg) => Ok(*reg),
                    None => Err(BytecodeGenerationError::NotFoundVariable {  }),
                }
            }
            Expr::Table { .. } => {
                if is_make_thunk {
                    let idx_thunk = self.make_thunk(expr.clone());
                    let reg_value = self
                        .get_register()
                        .expect("Error in get register: the value");
                    self.emit_ins(OpCode::MAKETHUNK.encode(vec![
                        reg_value as u32,
                        idx_thunk.try_into().expect("Error when convert idx_thunk to u32"),
                    ]));
                    Ok(reg_value)
                } else {
                    Ok(self.visit_table(expr.clone())?)
                }
            }
            Expr::List { .. } => {
                if is_make_thunk {
                    let idx_thunk = self.make_thunk(expr.clone());
                    let reg_value = self
                        .get_register()
                        .expect("Error in get register: the value");
                    self.emit_ins(OpCode::MAKETHUNK.encode(vec![
                        reg_value as u32,
                        idx_thunk.try_into().expect("Error when convert idx_thunk to u32"),
                    ]));
                    Ok(reg_value)
                } else {
                    Ok(self.visit_list(expr.clone())?)
                }
            }
            Expr::FunctionDeclare { body, params } => {
                let idx_func = self.make_function(body.clone(), params.clone());

                let reg_value = self
                    .get_register()
                    .expect("Error in get register: the value");

                self.emit_ins(OpCode::MAKEFUNC.encode(vec![
                    reg_value as u32,
                    idx_func.try_into().expect("Error when convert idx_func to u32"),
                ]));

                Ok(reg_value)
            }
            Expr::FunctionCall { name, args } => {
                if let Some(function_ref) = self.global_functions.get(name) {
                    let function = function_ref.clone();

                    if !self.global_function_in_used.contains_key(name) {
                        let idx_func =
                        self.make_function(function.body.clone(), function.params.clone());

                        self.global_function_in_used.insert(name.clone(), idx_func);
                    }


                    if args.len() != function.params.len() {
                        panic!(
                            "Function {} have {} params, but call it with {} arg",
                            name,
                            function.params.len(),
                            args.len(),
                        )
                    };

                    for arg in args {
                        let reg_arg = &self.visit_expr(arg, false)?;

                        self.emit_ins(OpCode::PUSHARG.encode(vec![*reg_arg as u32]));

                        self.free_register(*reg_arg as usize);
                    }

                    let reg_func_name =
                        self.get_register().expect("Error in get register: function name");

                    let idx_reg_func_name = self.make_const(Constant::String(name.clone()));

                    self.emit_ins(
                        OpCode::LOADCONST.encode(vec![
                            reg_func_name as u32,
                            idx_reg_func_name
                                .try_into()
                                .expect("Error when convert idx_const to u32"),
                        ]),
                    );

                    let reg_dist_result =
                        self.get_register().expect("Error in get register: dist result");

                    self.emit_ins(
                        OpCode::CALL.encode(vec![
                            reg_dist_result as u32,
                            reg_func_name as u32,
                        ]),
                    );

                    self.free_register(reg_func_name as usize);
                    self.free_register(reg_dist_result as usize);
                    Ok(reg_dist_result)
                } else {
                    Err(BytecodeGenerationError::NotFoundFunction {  })
                }
            }
            expr => panic!("Error: emit_expr, not implement yet {:?}", expr),
        }
    }

    fn visit_table(&mut self, expr: Expr) -> Result<u8, BytecodeGenerationError> {
        if let Expr::Table { fields } = expr {
            let reg_table = self.get_register().expect("Error in get register: table");
            self.emit_ins(OpCode::MAKETABLE.encode(vec![reg_table as u32]));

            for (key, value) in fields.iter() {
                let reg_key = self.get_register().expect("Error in get register: the key");

                let idx_const = self.make_const(Constant::String(key.clone()));

                self.emit_ins(OpCode::LOADCONST.encode(vec![
                    reg_key as u32,
                    idx_const.try_into().expect("Error when convert idx_const to u32"),
                ]));

                let reg_value = self.visit_expr(value, true)?;

                self.emit_ins(OpCode::SETATTR.encode(vec![
                    reg_table as u32,
                    reg_key as u32,
                    reg_value as u32,
                ]));

                self.free_register(reg_key as usize);
                self.free_register(reg_value as usize);
            }

            self.free_register(reg_table as usize);
            Ok(reg_table)
        } else {
            Err(BytecodeGenerationError::UnexpectExpr { message: "Expect Table".into() })
        }
    }

    fn visit_list(&mut self, expr: Expr) -> Result<u8, BytecodeGenerationError>{
        if let Expr::List { items } = expr {
            let reg_list = self.get_register().expect("Error in get register: list");
            self.emit_ins(OpCode::MAKELIST.encode(vec![reg_list as u32]));

            for value in items {
                let reg_value = self.visit_expr(&value, true)?;

                self.emit_ins(OpCode::ADDLIST.encode(vec![reg_list as u32, reg_value as u32]));

                self.free_register(reg_value as usize);
            }

            self.free_register(reg_list as usize);
            Ok(reg_list)
        } else {
            Err(BytecodeGenerationError::UnexpectExpr { message: "Expect List".into() })
        }
    }

    fn visit_function(&mut self) ->Result<(), BytecodeGenerationError>{
        let mut idx = 0;
        while idx < self.functions.len() {
            self.set_offset_func(idx, self.ins_count);
            let function = self.functions[idx].clone();
            let mut reg_params: Vec<usize> = vec![];

            if !self.context_var.is_empty() {
                panic!("Context var not empty");
            }

            for param in function.params {
                let reg_param = self
                    .get_register()
                    .expect("Error in get register: params function");
                self.context_var.insert(param.clone(), reg_param);
                self.emit_ins(OpCode::LOADARG.encode(vec![reg_param as u32]));
                reg_params.push(reg_param as usize);
            }

            let reg_value = self.visit_expr(&function.body,false)?;

            self.free_registers(reg_params);
            self.emit_ins(OpCode::RETURN.encode(vec![reg_value as u32]));
            self.free_register(reg_value as usize);
            self.context_var.clear();

            idx += 1;
        }
        Ok(())
    }

    pub fn add_const_to_binary(&mut self, bytes: &mut Vec<u8>) {
        let mut sort_const: Vec<(&Constant, &usize)> = self.constants.iter().collect();

        sort_const.sort_by(|a, b| a.1.cmp(b.1));

        for (constant, _) in sort_const {
            match constant {
                Constant::Number(num) => {
                    bytes.extend_from_slice(&0_u8.to_be_bytes());
                    bytes.extend_from_slice(&(*num as i64).to_be_bytes());
                }
                Constant::String(str) => {
                    bytes.extend_from_slice(&1_u8.to_be_bytes());
                    bytes.extend_from_slice(&(str.len() as u32).to_be_bytes());
                    bytes.extend_from_slice(&str.clone().into_bytes());
                }
            }
        }
    }

    pub fn add_header_to_binary(&self, bytes: &mut Vec<u8>) {
        let const_len_in_bytes = self.get_constant_len_in_bytes();
        let thunk_offset = const_len_in_bytes + METADATA_BYTES;
        let function_offset = thunk_offset + (self.thunks.len() as u32 * 4);
        let global_func_table_offset = function_offset + (self.global_function_in_used.len() as u32 * 8);
        let code_offset = function_offset + (self.functions.len() as u32 * 8);

        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
        bytes.extend_from_slice(&VERSION.to_be_bytes());

        bytes.extend_from_slice(&METADATA_BYTES.to_be_bytes()); // const offset in byte
        bytes.extend_from_slice(&(self.constants.len() as u32).to_be_bytes()); // const size

        bytes.extend_from_slice(&(thunk_offset.to_be_bytes())); // thunk_table offset in byte
        bytes.extend_from_slice(&(self.thunks.len() as u32).to_be_bytes()); // thunk_table size

        bytes.extend_from_slice(&(function_offset.to_be_bytes())); // function_table offset in byte
        bytes.extend_from_slice(&(self.functions.len() as u32).to_be_bytes()); // fucntion_table size

        bytes.extend_from_slice(&(global_func_table_offset.to_be_bytes())); 
        bytes.extend_from_slice(&(self.global_function_in_used.len() as u32).to_be_bytes());
        
        bytes.extend_from_slice(&code_offset.to_be_bytes()); // code offset in byte
        bytes.extend_from_slice(&self.ins_count.to_be_bytes()); // code size
    }

    pub fn add_ins_code_to_binary(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.ins_code);
    }

    pub fn add_thunk_table_to_binary(&mut self, bytes: &mut Vec<u8>) {
        for thunk in &self.thunks {
            bytes.extend_from_slice(&thunk.offset.to_be_bytes()); // code size
        }
    }

    pub fn add_function_table_to_binary(&mut self, bytes: &mut Vec<u8>) {
        for function in &self.functions {
            let nargs = function.params.len() as u32;
            bytes.extend_from_slice(&nargs.to_be_bytes()); // code size
            bytes.extend_from_slice(&function.offset.to_be_bytes()); // code size
        }
    }

    pub fn add_global_function_table_to_binary(&mut self, bytes: &mut Vec<u8>) {
        for (function_name, idx) in self.global_function_in_used.clone() {
            let function_name_const_idx = self.make_const(Constant::String(function_name.clone()));
            bytes.extend_from_slice(&(function_name_const_idx as u32).to_be_bytes()); // code size
            bytes.extend_from_slice(&(idx as u32).to_be_bytes());
        }
    }

    pub fn add_footer_to_binary(&mut self, bytes: &mut Vec<u8>) {
        let const_len_in_bytes = self.get_constant_len_in_bytes();

        let mut total_byte: u32 = 0;
        total_byte += METADATA_BYTES;
        total_byte += const_len_in_bytes;
        total_byte += self.thunks.len() as u32 * 4;
        total_byte += self.functions.len() as u32 * 8;
        total_byte += self.global_function_in_used.len() as u32 * 8;
        total_byte += self.ins_count * 4;
        total_byte += 4; // total_byte byte

        bytes.extend_from_slice(&total_byte.to_be_bytes());
    }

    pub fn visit_remain_thunk(&mut self) -> Result<(), BytecodeGenerationError>{
        let mut idx = 0;
        while idx < self.thunks.len() {
            match self.thunks[idx].expr {
                Expr::Table { .. } => {
                    self.set_offset_thunk(idx, self.ins_count);
                    self.visit_table(self.thunks[idx].expr.clone())?;
                }
                Expr::List { .. } => {
                    self.set_offset_thunk(idx, self.ins_count);
                    self.visit_list(self.thunks[idx].expr.clone())?;
                }
                _ => panic!("Not Impliment Yet"),
            }
            idx += 1;
        }
        Ok(())
    }

    pub fn make_thunk(&mut self, expr: Expr) -> usize {
        let idx = self.thunks.len() + 1;
        self.thunks.push(Thunk { expr, offset: 0 });
        idx
    }

    pub fn make_function(&mut self, body: Box<Expr>, params: Vec<String>) -> usize {
        let idx = self.functions.len() + 1;
        self.functions.push(Function {
            params,
            body,
            offset: 0,
        });
        idx
    }

    pub fn set_offset_thunk(&mut self, idx: usize, offset: u32) {
        let thunk = self
            .thunks
            .get_mut(idx)
            .expect("Error: not found thunk at index");
        thunk.offset = offset;
    }

    pub fn set_offset_func(&mut self, idx: usize, offset: u32) {
        let function = self
            .functions
            .get_mut(idx)
            .expect("Error: not found function at index");
        function.offset = offset;
    }

    pub fn make_const(&mut self, constant: Constant) -> usize {
        let idx = self.constants.len() + 1;
        let result = self.constants.entry(constant).or_insert(idx);
        *result
    }

    pub fn get_register(&mut self) -> Option<u8> {
        for i in 0..16 {
            if !self.register_in_used[i] {
                self.register_in_used[i] = true;
                return Some(i as u8);
            }
        }
        None
    }

    pub fn free_register(&mut self, idx: usize) {
        self.register_in_used[idx] = false;
    }

    pub fn free_registers(&mut self, idx_vec: Vec<usize>) {
        for idx in idx_vec {
            self.register_in_used[idx] = false;
        }
    }

    pub fn emit_ins(&mut self, bytes: [u8; 4]) -> usize {
        self.ins_code.extend_from_slice(&bytes);
        self.ins_count += 1;
        self.ins_count as usize
    }

    pub fn get_constant_len_in_bytes(&self) -> u32 {
        let mut const_len = 0;
        for constant in self.constants.keys() {
            match constant {
                Constant::Number(_) => {
                    const_len += 9; // 1 byte type + 8 byte num
                }
                Constant::String(str) => {
                    const_len += 5 + str.len() as u32
                    // 1 byte type + 4 byte len + len of str
                }
            }
        }
        const_len
    }
}
