use super::vm::Instruction;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Parser {
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }

    fn strip_comments<'a>(line: &'a str) -> &'a str {
        if let Some(pos) = line.find(';') {
            &line[..pos]
        } else {
            line
        }
    }

    pub fn parse_file(&mut self, contents: &str) -> Vec<Instruction> {
        // Primera pasada: almacenar etiquetas y sus índices

        let predicate = |line: &&str| !Parser::strip_comments(line.trim()).is_empty();
        let contents = contents.lines().filter(predicate);

        let mut labels = 0;
        for (ix, line) in contents.clone().enumerate() {
            if line.ends_with(':') {
                if self.labels.contains_key(line.trim_end_matches(':')) {
                    panic!("Etiqueta duplicada: {}", line);
                } else {
                    let n_ins = if (ix as i32 - labels as i32) < 0 {
                        0
                    } else {
                        ix - labels
                    };
                    self.labels
                        .insert(line.trim_end_matches(':').to_string(), n_ins as usize);
                    labels = labels + 1;
                }
            }
        }
        for (_, line) in contents.enumerate() {
            let instr = self.parse_line(line); // index y no len
            if let Some(instruction) = instr {
                self.instructions.push(instruction);
            }
        }

        // Segunda pasada: resolver etiquetas para instrucciones de salto
        // self.resolve_labels();
        self.instructions.clone()
    }

    fn parse_line(&mut self, line: &str) -> Option<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "LOAD_CONST" => {
                if parts.len() < 2 {
                    panic!("La instrucción LOAD_CONST requiere un operando numérico");
                }
                if let Ok(val) = f64::from_str(parts[1]) {
                    if val.fract() == 0.0 {
                        Some(Instruction::LoadConstInt(val as i64))
                    } else {
                        Some(Instruction::LoadConstFloat(val))
                    }
                } else {
                    panic!("LOAD_CONST requiere un número válido");
                }
            }
            "LOAD_VAR" => {
                if parts.len() < 2 {
                    panic!("La instrucción LOAD_VAR requiere un nombre de variable");
                }
                Some(Instruction::LoadVar(parts[1].to_string()))
            }
            "STORE_VAR" => {
                if parts.len() < 2 {
                    panic!("La instrucción STORE_VAR requiere un nombre de variable");
                }
                Some(Instruction::StoreVar(parts[1].to_string()))
            }
            "ADD" => Some(Instruction::Add),
            "SUB" => Some(Instruction::Sub),
            "MUL" => Some(Instruction::Mul),
            "DIV" => Some(Instruction::Div),
            "PRINT" => Some(Instruction::Print),
            "READ" => Some(Instruction::Read),
            "POW" => Some(Instruction::Pow),
            "MOD" => Some(Instruction::Mod),
            "JMP" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMP requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::Jmp(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPEQ" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPEQ requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpEq(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPNE" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPNE requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpNe(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPGT" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPGT requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpGt(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPLT" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPLT requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpLt(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPGE" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPGE requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpGe(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            "JMPLE" => {
                if parts.len() < 2 {
                    panic!("La instrucción JMPLE requiere una etiqueta");
                }
                match self.labels.get(parts[1]) {
                    Some(ix) => Some(Instruction::JmpLe(ix.to_owned())),
                    None => panic!("Etiqueta no encontrada: {}", parts[1]),
                }
            }
            _ => {
                if line.ends_with(':') {
                    None
                } else {
                    panic!("Instrucción desconocida: {}", parts[0]);
                }
            }
        }
    }
}
