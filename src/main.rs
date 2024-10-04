use std::io::{ self, Write };

const AC: usize = 0; //Accumulator
const PC: usize = 1; //Program Counter
const IR: usize = 2; //Instruction Register
const MAR: usize = 3; //Memory Address Register
const MBR: usize = 4; //Memory Buffer Register
// const INPUT: usize = 5; input register - implemented via std::io
// const OUTPUT: usize = 6; output register - implemented via std::io

struct CPU {
    registers: [i16; 7],
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 7],
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
                let cond = self.registers[IR] & (0x0c00 >> 10); //Get condition from bits 11-10 and shift to LSB for easy comparison
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
    AddI,
    JumpI,
    LoadI,
    StoreI,
}
*/

fn main() {
    let mut memory = Memory::new(4096);

    let program = vec![
        0x101b, // Load H (ASCII 72) into AC
        0x6000, // Output AC
        0x101c, // Load e (ASCII 101) into AC
        0x6000, // Output AC
        0x101d, // Load l (ASCII 108) into AC
        0x6000, // Output AC
        0x101d, // Load l (ASCII 108) into AC
        0x6000, // Output AC
        0x101e, // Load o (ASCII 111) into AC
        0x6000, // Output AC
        0x101f, // Load , (ASCII 44) into AC
        0x6000, // Output AC
        0x1020, // Load space (ASCII 32) into AC
        0x6000, // Output AC
        0x1021, // Load W (ASCII 87) into AC
        0x6000, // Output AC
        0x101e, // Load o (ASCII 111) into AC
        0x6000, // Output AC
        0x1022, // Load r (ASCII 114) into AC
        0x6000, // Output AC
        0x101d, // Load l (ASCII 108) into AC
        0x6000, // Output AC
        0x1023, // Load d (ASCII 100) into AC
        0x6000, // Output AC
        0x1024, // Load ! (ASCII 33) into AC
        0x6000, // Output AC
        0x7000, // Halt
        // Data section
        0x0048, // H (ASCII 72) - addr 0x001b
        0x0065, // e (ASCII 101) - addr 0x001c
        0x006c, // l (ASCII 108) - addr 0x001d
        0x006f, // o (ASCII 111) - addr 0x001e
        0x002c, // , (ASCII 44) - addr 0x001f
        0x0020, // space (ASCII 32) - addr 0x0020
        0x0057, // W (ASCII 87) - addr 0x0021
        0x0072, // r (ASCII 114) - addr 0x0022
        0x0064, // d (ASCII 100) - addr 0x0023
        0x0021 // ! (ASCII 33) - addr 0x0024
    ];
    memory.load_program(program);

    let mut cpu = CPU::new();
    cpu.run(&mut memory);
}
