use std::collections::HashMap;
use std::io::{self};

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConstFloat(f64),
    LoadConstInt(i64),
    LoadVar(String),
    StoreVar(String),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Print,
    Read,
    Jmp(usize),
    JmpEq(usize),
    JmpNe(usize),
    JmpGe(usize),
    JmpGt(usize),
    JmpLt(usize),
    JmpLe(usize),
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
}

pub struct VM {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    instructions: Vec<Instruction>,
    ip: usize, // Instruction pointer
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        VM {
            stack: Vec::new(),
            vars: HashMap::new(),
            instructions,
            ip: 0,
        }
    }

    pub fn current_instruction(&self) -> Option<&Instruction> {
        self.instructions.get(self.ip)
    }

    pub fn print_stack(&self) {
        println!("{:<5} | {:<10}", "Index", "Value");
        println!("---------------------");
        for (i, val) in self.stack.iter().enumerate() {
            match val {
                Value::Float(f) => println!("{:<5} | {:<10}", i, f),
                Value::Int(i) => println!("{:<5} | {:<10}", i, i),
            }
        }
    }

    pub fn print_vars(&self) {
        println!("{:<10} | {:<10}", "Variable", "Value");
        println!("--------------------------");
        for (name, val) in &self.vars {
            match val {
                Value::Float(f) => println!("{:<10} | {:<10}", name, f),
                Value::Int(i) => println!("{:<10} | {:<10}", name, i),
            }
        }
    }

    pub fn step(&mut self) {
        let instr = &self.instructions[self.ip].clone();

        match instr {
            Instruction::LoadConstFloat(val) => self.stack.push(Value::Float(*val)),
            Instruction::LoadConstInt(val) => self.stack.push(Value::Int(*val)),
            Instruction::LoadVar(name) => {
                if let Some(val) = self.vars.get(name) {
                    self.stack.push(val.clone());
                } else {
                    panic!("Variable {} not found", name);
                }
            }
            Instruction::StoreVar(name) => {
                if let Some(val) = self.stack.pop() {
                    self.vars.insert(name.clone(), val);
                } else {
                    panic!("Stack underflow");
                }
            }
            Instruction::Add => self.binary_op(|a, b| a + b),
            Instruction::Sub => self.binary_op(|a, b| a - b),
            Instruction::Mul => self.binary_op(|a, b| a * b),
            Instruction::Div => self.binary_op(|a, b| a / b),
            Instruction::Pow => self.binary_op(|a, b| a.powf(b)),
            Instruction::Mod => self.binary_op(|a, b| a % b),
            Instruction::Print => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Float(f) => println!("{}", f),
                        Value::Int(i) => println!("{}", i),
                    }
                } else {
                    panic!("Stack underflow on print");
                }
            }
            Instruction::Read => {
                let mut input = String::new();
                println!("Programa solicita entrada: ");
                io::stdin().read_line(&mut input).expect("Failed to read");
                if let Ok(val) = input.trim().parse::<f64>() {
                    self.stack.push(Value::Float(val));
                } else if let Ok(val) = input.trim().parse::<i64>() {
                    self.stack.push(Value::Int(val));
                } else {
                    panic!("Invalid input");
                }
            }
            Instruction::Jmp(target) => {
                self.ip = *target;
                return;
            }
            Instruction::JmpEq(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f == 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i == 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
            Instruction::JmpNe(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f != 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i != 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
            Instruction::JmpGe(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f >= 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i >= 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
            Instruction::JmpGt(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f > 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i > 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
            Instruction::JmpLt(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f < 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i < 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
            Instruction::JmpLe(target) => {
                if let Some(cond) = self.stack.pop() {
                    match cond {
                        Value::Float(f) if f <= 0.0 => {
                            self.ip = *target;
                            return;
                        }
                        Value::Int(i) if i <= 0 => {
                            self.ip = *target;
                            return;
                        }
                        _ => {}
                    }
                } else {
                    panic!("Stack underflow on conditional jump");
                }
            }
        }
        self.ip += 1;
    }

    pub fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.step();
        }
    }

    pub fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            let result = match (a, b) {
                (Value::Float(a), Value::Float(b)) => Value::Float(op(a, b)),
                (Value::Int(a), Value::Int(b)) => Value::Int(op(a as f64, b as f64) as i64),
                (Value::Float(a), Value::Int(b)) => Value::Float(op(a, b as f64)),
                (Value::Int(a), Value::Float(b)) => Value::Float(op(a as f64, b)),
            };
            self.stack.push(result);
        } else {
            panic!("Stack underflow on binary operation");
        }
    }
}
