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
        Ok(machine.drain_output())
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

    pub fn push_text_input(&mut self, input: impl AsRef<str>) {
        #![allow(dead_code)]
        for input in input.as_ref().chars().map(|c| c as i64) {
            self.push_input(input);
        }
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        #![allow(dead_code)]
        self.output_buffer.pop_front()
    }

    pub fn drain_output(&mut self) -> Vec<i64> {
        Vec::from_iter(self.output_buffer.drain(..))
    }

    pub fn step(&mut self) -> Result<State> {
        let instruction = self.read(self.instruction_pointer)?;
        let instruction = Instruction::decode(self.instruction_pointer, instruction)?;
        match instruction.opcode {
            1 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let value = a + b;
                self.address_write(
                    instruction.parameter_modes[2],
                    self.instruction_pointer + 3,
                    value,
                )?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            2 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let value = a * b;
                self.address_write(
                    instruction.parameter_modes[2],
                    self.instruction_pointer + 3,
                    value,
                )?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            3 => {
                let Some(input) = self.input_buffer.pop_front() else {
                    return Ok(State::WaitingForInput);
                };

                self.address_write(
                    instruction.parameter_modes[0],
                    self.instruction_pointer + 1,
                    input,
                )?;

                self.instruction_pointer += 2;
                Ok(State::Running)
            }
            4 => {
                let output = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                self.output_buffer.push_back(output);

                self.instruction_pointer += 2;
                Ok(State::Running)
            }
            5 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;

                if a != 0 {
                    self.instruction_pointer = b;
                } else {
                    self.instruction_pointer += 3;
                }

                Ok(State::Running)
            }
            6 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;

                if a == 0 {
                    self.instruction_pointer = b;
                } else {
                    self.instruction_pointer += 3;
                }

                Ok(State::Running)
            }
            7 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let value = if a < b { 1 } else { 0 };
                self.address_write(
                    instruction.parameter_modes[2],
                    self.instruction_pointer + 3,
                    value,
                )?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            8 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
                let b = self
                    .address_read(instruction.parameter_modes[1], self.instruction_pointer + 2)?;
                let value = if a == b { 1 } else { 0 };
                self.address_write(
                    instruction.parameter_modes[2],
                    self.instruction_pointer + 3,
                    value,
                )?;

                self.instruction_pointer += 4;
                Ok(State::Running)
            }
            9 => {
                let a = self
                    .address_read(instruction.parameter_modes[0], self.instruction_pointer + 1)?;
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

    fn get_address(&self, mode: AddressingMode, address: i64) -> Result<i64> {
        if address < 0 {
            return Err(Error::IllegalMemoryAccess {
                position: self.instruction_pointer,
                address,
            });
        }

        match mode {
            AddressingMode::Position => Ok(self.memory.read(address as usize)),
            AddressingMode::Immediate => Ok(address),
            AddressingMode::Relative => Ok(self.memory.read(address as usize) + self.relative_base),
        }
    }

    fn address_read(&self, mode: AddressingMode, address: i64) -> Result<i64> {
        let address = self.get_address(mode, address)?;
        self.read(address)
    }

    fn address_write(&mut self, mode: AddressingMode, address: i64, value: i64) -> Result<()> {
        let address = self.get_address(mode, address)?;
        self.write(address, value)
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
        #![allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzle::Result;
    use rstest::*;

    fn input(day: usize, which: usize) -> Result<String> {
        let file = format!("inputs/day{:02}/test.{}.txt", day, which);
        let file = std::fs::read_to_string(file)?;
        Ok(file)
    }

    #[rstest]
    #[case(5, 0, [0], 0)]
    #[case(5, 0, [5], 1)]
    #[case(5, 1, [0], 0)]
    #[case(5, 1, [5], 1)]
    #[case(5, 2, [1], 999)]
    #[case(5, 2, [8], 1000)]
    #[case(5, 2, [50], 1001)]
    #[case(9, 1, [], 1219070632396864)]
    #[case(9, 2, [], 1125899906842624)]
    fn test_single_output(
        #[case] day: usize,
        #[case] which: usize,
        #[case] program_input: impl IntoIterator<Item = i64>,
        #[case] expected: i64,
    ) -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(day, which)?;

        let program = parse_program(&input)?;
        let result = Intcode::run_program_with_inputs(program, program_input)?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
        Ok(())
    }

    #[rstest]
    fn test_quine() -> Result<()> {
        crate::util::test::setup_tracing();
        let input = input(9, 0)?;

        let program = parse_program(&input)?;
        let result = Intcode::run_program_with_inputs(&program, [])?;

        assert_eq!(program.as_ref(), result.as_slice());
        Ok(())
    }
}
