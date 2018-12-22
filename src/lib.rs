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

    pub fn push_code(&mut self, code: u8, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn push_constant(&mut self, v: Value) -> u8 {
        self.constants.push(v);
        (self.constants.len() - 1) as u8
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
