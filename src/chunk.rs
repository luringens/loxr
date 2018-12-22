use crate::{OpCode, Value};

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<u32>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    #[inline]
    pub fn push_code(&mut self, code: u8, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    #[inline]
    pub fn push_constant(&mut self, v: Value) -> u8 {
        self.constants.push(v);
        (self.constants.len() - 1) as u8
    }

    #[inline]
    pub fn read_constant(&self, i: u8) -> Option<&Value> {
        self.constants.get(i as usize)
    }

    #[inline]
    pub fn get_code(&self) -> &[u8] {
        &self.code
    }

    pub fn disassemble(&self) {
        let mut i = 0;
        while i < self.code.len() {
            let code = OpCode::from(self.code[i]).unwrap();
            let line = i
                .checked_sub(1)
                .map(|i_prev| (self.lines[i_prev], self.lines[i]))
                .map(|(prev, current)| {
                    if prev == current {
                        "   |".to_owned()
                    } else {
                        format!("{:04}", current)
                    }
                })
                .unwrap_or_else(|| format!("{:04}", self.lines[i]));

            i += match code {
                OpCode::Constant => {
                    if i + 1 >= self.code.len() {
                        return;
                    }
                    println!(
                        "{:04} {} {:?} {:>10}",
                        i,
                        line,
                        code,
                        self.code[i + 1] as u8
                    );
                    2
                }
                _ => {
                    println!("{:04} {} {:?}", i, line, code);
                    1
                }
            }
        }
    }
}
