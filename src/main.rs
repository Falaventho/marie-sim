use std::{ fs, io::{ self, Write } };

const AC: usize = 0; //Accumulator
const PC: usize = 1; //Program Counter
const IR: usize = 2; //Instruction Register
const MAR: usize = 3; //Memory Address Register
const MBR: usize = 4; //Memory Buffer Register
// const INPUT: usize = 5; input register - implemented via std::io
// const OUTPUT: usize = 6; output register - implemented via std::io

struct CPU {
    registers: [i16; 5],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 5],
        }
    }

    fn fetch(&mut self, memory: &Memory) {
        self.registers[MAR] = self.registers[PC];
        self.registers[IR] = memory.read(self.registers[MAR]);
        self.registers[PC] += 1;
    }

    fn decode_execute(&mut self, memory: &mut Memory) {
        match self.registers[IR] >> 12 {
            //Get opcode from 4 high bits

            //JnS
            0x0 => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = self.registers[PC];
                memory.write(self.registers[MAR], self.registers[MBR]);
                self.registers[PC] = self.registers[IR] + 1;
            }

            //LOAD
            0x1 => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[AC] = memory.read(self.registers[MAR]);
            }

            //STORE
            0x2 => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = self.registers[AC];
                memory.write(self.registers[MAR], self.registers[MBR]);
            }

            //ADD
            0x3 => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] += self.registers[MBR];
            }

            //SUBT
            0x4 => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] -= self.registers[MBR];
            }

            //INPUT
            0x5 => {
                self.input();
            }

            //OUTPUT
            0x6 => {
                self.output();
            }

            //HALT
            0x7 => {
                std::process::exit(0);
            }

            //SKIPCOND
            0x8 => {
                let cond = (self.registers[IR] & 0x0c00) >> 10; //Get condition from bits 11-10 and shift to LSB for easy comparison
                match cond {
                    0b00 => {
                        if self.registers[AC] < 0 {
                            self.registers[PC] += 1;
                        }
                    }

                    0x01 => {
                        if self.registers[AC] == 0 {
                            self.registers[PC] += 1;
                        }
                    }

                    0b10 => {
                        if self.registers[AC] > 0 {
                            self.registers[PC] += 1;
                        }
                    }
                    _ => {}
                }
            }

            //JUMP
            0x9 => {
                self.registers[PC] = self.registers[IR] & 0x0fff;
            }

            //CLEAR
            0xa => {
                self.registers[AC] = 0x0000;
            }

            //ADDI
            0xb => {
                // read indirect
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] += self.registers[MBR];
            }

            //JUMPI
            0xc => {
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = self.registers[MAR];
                self.registers[PC] = self.registers[MBR];
            }

            //LOADI
            0xd => {
                // read indirect
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] = self.registers[MBR];
            }

            //STOREI
            0xe => {
                // write indirect
                self.registers[MAR] = self.registers[IR] & 0x0fff;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = self.registers[AC];
                memory.write(self.registers[MAR], self.registers[MBR]);
            }

            _ => {}
        }
    }

    fn input(&mut self) {
        let mut input = String::new();
        print!("Enter input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        self.registers[AC] = input.trim().parse().unwrap_or(0);
    }

    fn output(&self) {
        let character = (self.registers[AC] & 0x00ff) as u8 as char;
        print!("{}", character);
    }

    fn run(&mut self, memory: &mut Memory) {
        loop {
            self.fetch(memory);
            self.decode_execute(memory);
        }
    }
}

struct Memory {
    mem: Vec<i16>,
}

impl Memory {
    fn new(size: usize) -> Memory {
        Memory {
            mem: vec![0; size],
        }
    }

    fn read(&self, address: i16) -> i16 {
        self.mem[address as usize]
    }

    fn write(&mut self, address: i16, value: i16) {
        self.mem[address as usize] = value;
    }

    fn load_program(&mut self, program: Vec<i16>) {
        for (i, &instruction) in program.iter().enumerate() {
            self.write(i as i16, instruction);
        }
    }
}

/*
enum Instruction {
    JnS(i16),
    Load(i16),
    Store(i16),
    Add(i16),
    Subt(i16),
    Input,
    Output,
    Halt,
    Skipcond(i16),
    Jump(i16),
    Clear,
    AddI(i16),
    JumpI(i16),
    LoadI(i16),
    StoreI(i16),
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory = Memory::new(4096);

    let program_file = fs::read_to_string("./program.mrf")?;
    let raw_bytes = hex::decode(program_file.trim())?;
    let program: Vec<i16> = raw_bytes
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 2 {
                Ok((((chunk[0] as u16) << 8) | (chunk[1] as u16)) as i16)
            } else {
                Err("Incomplete byte pair")
            }
        })
        .collect::<Result<Vec<i16>, &str>>()?;

    memory.load_program(program);

    let mut cpu = CPU::new();
    cpu.run(&mut memory);

    Ok(())
}
