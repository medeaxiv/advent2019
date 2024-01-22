#[derive(Debug, Clone)]
pub struct Intcode {
    ip: usize,
    program: Box<[i64]>,
}

impl Intcode {
    pub fn new(program: impl AsRef<[i64]>) -> Self {
        Self {
            ip: 0,
            program: Box::from(program.as_ref()),
        }
    }

    pub fn step(&mut self) -> Result<State, Error> {
        let instruction = self.read(self.ip)?;
        let instruction = Instruction::decode(self.ip, instruction)?;
        match instruction.opcode {
            1 => {
                let a = self.address(instruction.parameter_modes[2], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                self.write(c, a + b)?;
                self.ip += 4;
                Ok(State::Running)
            }
            2 => {
                let a = self.address(instruction.parameter_modes[2], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                self.write(c, a * b)?;
                self.ip += 4;
                Ok(State::Running)
            }
            99 => Ok(State::Terminated),
            opcode => Err(Error::new(ErrorKind::UnknownOpcode {
                position: self.ip,
                opcode,
            })),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let state = self.step()?;

            if matches!(state, State::Terminated) {
                return Ok(());
            }
        }
    }

    fn address(&self, mode: AddressingMode, address: usize) -> Result<i64, Error> {
        if address >= self.program.len() {
            return Err(Error::new(ErrorKind::IllegalMemoryAccess {
                position: self.ip,
                address,
            }));
        }

        match mode {
            AddressingMode::Position => self.read(self.program[address] as usize),
            AddressingMode::Immediate => Ok(self.program[address]),
        }
    }

    fn read(&self, address: usize) -> Result<i64, Error> {
        self.program.get(address).copied().ok_or_else(|| {
            Error::new(ErrorKind::IllegalMemoryAccess {
                position: self.ip,
                address,
            })
        })
    }

    fn write(&mut self, address: usize, value: i64) -> Result<(), Error> {
        let memory = self.program.get_mut(address).ok_or_else(|| {
            Error::new(ErrorKind::IllegalMemoryAccess {
                position: self.ip,
                address,
            })
        })?;

        *memory = value;
        Ok(())
    }

    pub fn get_program(&self) -> &[i64] {
        &self.program
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Running,
    Terminated,
}

struct Instruction {
    opcode: u8,
    parameter_modes: [AddressingMode; 3],
}

impl Instruction {
    fn decode(position: usize, instruction: i64) -> Result<Self, Error> {
        let opcode = (instruction % 100) as u8;
        let parameter_modes = [
            AddressingMode::decode(position, 0, (instruction / 100 % 10) as u8)?,
            AddressingMode::decode(position, 1, (instruction / 1_000 % 10) as u8)?,
            AddressingMode::decode(position, 2, (instruction / 10_000 % 10) as u8)?,
        ];

        Ok(Self {
            opcode,
            parameter_modes,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddressingMode {
    Position,
    Immediate,
}

impl AddressingMode {
    fn decode(position: usize, parameter: u8, mode: u8) -> Result<Self, Error> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(Error::new(ErrorKind::InvalidParameterMode {
                position,
                parameter,
                mode,
            })),
        }
    }
}

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self(Box::new(kind))
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self.0 {
            ErrorKind::UnknownOpcode {
                ref position,
                ref opcode,
            } => {
                write!(f, "Intcode error: unknown opcode {opcode:0.2} @ {position}")
            }
            ErrorKind::InvalidParameterMode {
                ref position,
                ref parameter,
                ref mode,
            } => {
                write!(
                    f,
                    "Intcode error: invalid parameter mode {parameter} {mode} @ {position}"
                )
            }
            ErrorKind::IllegalMemoryAccess {
                ref position,
                ref address,
            } => {
                write!(
                    f,
                    "Intcode error: out of bounds memory access {address} @ {position}"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    UnknownOpcode {
        position: usize,
        opcode: u8,
    },
    InvalidParameterMode {
        position: usize,
        parameter: u8,
        mode: u8,
    },
    IllegalMemoryAccess {
        position: usize,
        address: usize,
    },
}