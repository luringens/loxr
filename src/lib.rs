mod chunk;
mod vm;

pub use crate::chunk::Chunk;
pub use crate::vm::VirtualMachine;

#[derive(Clone, Debug)]
pub enum Value {
    Number(i32),
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum OpCode {
    Return = 0000,
    Constant = 0001,
}

impl OpCode {
    fn from(n: u8) -> Result<OpCode, ()> {
        if n <= 1 {
            Ok(unsafe { std::mem::transmute(n) })
        } else {
            Err(())
        }
    }
}
