use loxr::*;

fn main() {
    let mut chunk = Chunk::new();
    let const_index = chunk.push_constant(Value::Number(42));
    chunk.push_code(OpCode::Constant as u8, 1);
    chunk.push_code(const_index, 1);
    chunk.push_code(OpCode::Return as u8, 1);
    chunk.disassemble();
}
