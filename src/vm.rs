use crate::{Chunk, OpCode, Value};

#[derive(Clone, Copy, Debug)]
pub enum InterpreterError {
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    chunk: Chunk,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk }
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        let code = self.chunk.get_code();
        let mut ip = 0;
        loop {
            let instruction = code
                .get(ip)
                .ok_or_else(|| InterpreterError::RuntimeError)
                .and_then(|i| OpCode::from(*i).map_err(|_| InterpreterError::RuntimeError))?;

            ip += 1;
            match instruction {
                OpCode::Return => return Ok(()),
                OpCode::Constant => {
                    let constant = code
                        .get(ip)
                        .and_then(|i| self.chunk.read_constant(*i))
                        .ok_or_else(|| InterpreterError::RuntimeError)?;
                    ip += 1;
                    println!("Constant: {:?}", constant);
                }
                _ => return Err(InterpreterError::RuntimeError),
            }
        }
    }
}
