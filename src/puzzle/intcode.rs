use std::{collections::VecDeque, num::ParseIntError};

pub fn parse_program(input: &str) -> core::result::Result<Box<[i64]>, ParseIntError> {
    input.split(',').map(|s| s.trim().parse::<i64>()).collect()
}

#[derive(Debug, Clone)]
pub struct Intcode {
    state: State,
    ip: usize,
    program: Box<[i64]>,
    input_buffer: VecDeque<i64>,
    output_buffer: VecDeque<i64>,
}

impl Intcode {
    pub fn run_program_with_inputs(
        program: impl AsRef<[i64]>,
        inputs: impl IntoIterator<Item = i64>,
    ) -> Result<Vec<i64>> {
        let mut machine = Self::new(program);
        machine.input_buffer.extend(inputs);
        machine.run()?;
        Ok(Vec::from_iter(machine.output_buffer.drain(..)))
    }
}

impl Intcode {
    pub fn new(program: impl AsRef<[i64]>) -> Self {
        Self {
            state: State::Initial,
            ip: 0,
            program: Box::from(program.as_ref()),
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, input: i64) {
        #![allow(dead_code)]
        self.input_buffer.push_back(input);
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        #![allow(dead_code)]
        self.output_buffer.pop_front()
    }

    pub fn step(&mut self) -> Result<State> {
        let instruction = self.read(self.ip)?;
        let instruction = Instruction::decode(self.ip, instruction)?;
        match instruction.opcode {
            1 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                self.write(c, a + b)?;

                self.ip += 4;
                Ok(State::Running)
            }
            2 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                self.write(c, a * b)?;

                self.ip += 4;
                Ok(State::Running)
            }
            3 => {
                let Some(input) = self.input_buffer.pop_front() else {
                    return Ok(State::WaitingForInput);
                };

                let a = self.read(self.ip + 1)? as usize;
                self.write(a, input)?;

                self.ip += 2;
                Ok(State::Running)
            }
            4 => {
                let output = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                self.output_buffer.push_back(output);

                self.ip += 2;
                Ok(State::Running)
            }
            5 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)? as usize;

                if a != 0 {
                    self.ip = b;
                } else {
                    self.ip += 3;
                }

                Ok(State::Running)
            }
            6 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)? as usize;

                if a == 0 {
                    self.ip = b;
                } else {
                    self.ip += 3;
                }

                Ok(State::Running)
            }
            7 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                if a < b {
                    self.write(c, 1)?;
                } else {
                    self.write(c, 0)?;
                }

                self.ip += 4;
                Ok(State::Running)
            }
            8 => {
                let a = self.address(instruction.parameter_modes[0], self.ip + 1)?;
                let b = self.address(instruction.parameter_modes[1], self.ip + 2)?;
                let c = self.read(self.ip + 3)? as usize;
                if a == b {
                    self.write(c, 1)?;
                } else {
                    self.write(c, 0)?;
                }

                self.ip += 4;
                Ok(State::Running)
            }
            99 => Ok(State::Terminated),
            opcode => Err(Error::UnknownOpcode {
                position: self.ip,
                opcode,
            }),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let state = self.step()?;

            if !matches!(state, State::Running) {
                self.state = state;
                return Ok(());
            }
        }
    }

    fn address(&self, mode: AddressingMode, address: usize) -> Result<i64> {
        if address >= self.program.len() {
            return Err(Error::IllegalMemoryAccess {
                position: self.ip,
                address,
            });
        }

        match mode {
            AddressingMode::Position => self.read(self.program[address] as usize),
            AddressingMode::Immediate => Ok(self.program[address]),
        }
    }

    fn read(&self, address: usize) -> Result<i64> {
        self.program
            .get(address)
            .copied()
            .ok_or(Error::IllegalMemoryAccess {
                position: self.ip,
                address,
            })
    }

    fn write(&mut self, address: usize, value: i64) -> Result<()> {
        let memory = self
            .program
            .get_mut(address)
            .ok_or(Error::IllegalMemoryAccess {
                position: self.ip,
                address,
            })?;

        *memory = value;
        Ok(())
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_program(&self) -> &[i64] {
        &self.program
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    Running,
    WaitingForInput,
    Terminated,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: u8,
    parameter_modes: [AddressingMode; 3],
}

impl Instruction {
    fn decode(position: usize, instruction: i64) -> Result<Self> {
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
    fn decode(position: usize, parameter: u8, mode: u8) -> Result<Self> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(Error::InvalidParameterMode {
                position,
                parameter,
                mode,
            }),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Intcode error: missing output")]
    MissingOutput,
    #[error("Intcode error: unknown opcode {opcode:0.2} @ {position}")]
    UnknownOpcode { position: usize, opcode: u8 },
    #[error("Intcode error: invalid parameter mode {parameter} {mode} @ {position}")]
    InvalidParameterMode {
        position: usize,
        parameter: u8,
        mode: u8,
    },
    #[error("Intcode error: out of bounds memory access {address} @ {position}")]
    IllegalMemoryAccess { position: usize, address: usize },
}
