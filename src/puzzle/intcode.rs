use std::{collections::VecDeque, num::ParseIntError};

use ahash::AHashMap as HashMap;

pub fn parse_program(input: &str) -> core::result::Result<Box<[i64]>, ParseIntError> {
    input.split(',').map(|s| s.trim().parse::<i64>()).collect()
}

#[derive(Debug, Clone)]
pub struct Intcode {
    state: State,
    instruction_pointer: i64,
    relative_base: i64,
    memory: Memory,
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
            instruction_pointer: 0,
            relative_base: 0,
            memory: Memory::from(program),
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
        let instruction = self.read(self.instruction_pointer)?;
        let instruction = Instruction::decode(self.instruction_pointer, instruction)?;
        match instruction.opcode {
            1 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let c = self.read(self.instruction_pointer + 3)?;
                self.write(c, a + b)?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            2 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let c = self.read(self.instruction_pointer + 3)?;
                self.write(c, a * b)?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            3 => {
                let Some(input) = self.input_buffer.pop_front() else {
                    return Ok(State::WaitingForInput);
                };

                let a = self.read(self.instruction_pointer + 1)?;
                self.write(a, input)?;

                self.instruction_pointer += 2;
                Ok(State::Running)
            }
            4 => {
                let output =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                self.output_buffer.push_back(output);

                self.instruction_pointer += 2;
                Ok(State::Running)
            }
            5 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;

                if a != 0 {
                    self.instruction_pointer = b;
                } else {
                    self.instruction_pointer += 3;
                }

                Ok(State::Running)
            }
            6 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;

                if a == 0 {
                    self.instruction_pointer = b;
                } else {
                    self.instruction_pointer += 3;
                }

                Ok(State::Running)
            }
            7 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let c = self.read(self.instruction_pointer + 3)?;
                if a < b {
                    self.write(c, 1)?;
                } else {
                    self.write(c, 0)?;
                }

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            8 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b =
                    self.address(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let c = self.read(self.instruction_pointer + 3)?;
                if a == b {
                    self.write(c, 1)?;
                } else {
                    self.write(c, 0)?;
                }

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            9 => {
                let a =
                    self.address(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                self.relative_base += a;

                self.instruction_pointer += 2;
                Ok(State::Running)
            }
            99 => Ok(State::Terminated),
            opcode => Err(Error::UnknownOpcode {
                position: self.instruction_pointer,
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

    fn address(&self, mode: AddressingMode, address: i64) -> Result<i64> {
        if address < 0 {
            return Err(Error::IllegalMemoryAccess {
                position: self.instruction_pointer,
                address,
            });
        }

        match mode {
            AddressingMode::Position => self.read(self.memory.read(address as usize)),
            AddressingMode::Immediate => Ok(self.memory.read(address as usize)),
            AddressingMode::Relative => {
                self.read(self.memory.read(address as usize) + self.relative_base)
            }
        }
    }

    fn read(&self, address: i64) -> Result<i64> {
        if address < 0 {
            return Err(Error::IllegalMemoryAccess {
                position: self.instruction_pointer,
                address,
            });
        }

        Ok(self.memory.read(address as usize))
    }

    fn write(&mut self, address: i64, value: i64) -> Result<()> {
        if address < 0 {
            return Err(Error::IllegalMemoryAccess {
                position: self.instruction_pointer,
                address,
            });
        }

        self.memory.write(address as usize, value);
        Ok(())
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    pages: HashMap<usize, Box<[i64; Self::PAGE_SIZE]>>,
}

impl Memory {
    const PAGE_BITS: usize = 8;
    const PAGE_SIZE: usize = 1 << Self::PAGE_BITS;
    const PAGE_MASK: usize = usize::MAX >> (usize::BITS as usize - Self::PAGE_BITS);

    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    pub fn from_buffer(buffer: impl AsRef<[i64]>) -> Self {
        let buffer = buffer.as_ref();
        let mut pages = HashMap::new();

        for (page_idx, chunk) in buffer.chunks(Self::PAGE_SIZE).enumerate() {
            let mut page = Self::new_page();
            page[0..chunk.len()].copy_from_slice(chunk);
            pages.insert(page_idx, page);
        }

        Self { pages }
    }

    fn new_page() -> Box<[i64; Self::PAGE_SIZE]> {
        Box::new([0; Self::PAGE_SIZE])
    }

    pub fn read(&self, address: usize) -> i64 {
        let page_idx = address >> Self::PAGE_BITS;
        let page_address = address & Self::PAGE_MASK;

        if let Some(page) = self.pages.get(&page_idx) {
            page[page_address]
        } else {
            0
        }
    }

    pub fn write(&mut self, address: usize, value: i64) {
        let page_idx = address >> Self::PAGE_BITS;
        let page_address = address & Self::PAGE_MASK;

        let page = self.pages.entry(page_idx).or_insert_with(Self::new_page);
        page[page_address] = value;
    }
}

impl<T> From<T> for Memory
where
    T: AsRef<[i64]>,
{
    fn from(value: T) -> Self {
        Self::from_buffer(value)
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
    fn decode(position: i64, instruction: i64) -> Result<Self> {
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
    Relative,
}

impl AddressingMode {
    fn decode(position: i64, parameter: u8, mode: u8) -> Result<Self> {
        match mode {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
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
    UnknownOpcode { position: i64, opcode: u8 },
    #[error("Intcode error: invalid parameter mode {parameter} {mode} @ {position}")]
    InvalidParameterMode {
        position: i64,
        parameter: u8,
        mode: u8,
    },
    #[error("Intcode error: out of bounds memory access {address} @ {position}")]
    IllegalMemoryAccess { position: i64, address: i64 },
}
