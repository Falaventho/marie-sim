const AC: usize = 0; //Accumulator
const PC: usize = 1; //Program Counter
const IR: usize = 2; //Instruction Register
const MAR: usize = 3; //Memory Address Register
const MBR: usize = 4; //Memory Buffer Register
const INPUT: usize = 5;
const OUTPUT: usize = 6;


struct CPU {
    registers: [u16; 7],
}

impl CPU {

    fn fetch(&mut self, memory: &Memory) {
        self.registers[MAR] = self.registers[PC];
        self.registers[IR] = memory.read(self.registers[MAR] as usize);
        self.registers[PC] += 1;
    }


    fn decode_execute(&mut self, memory: &mut Memory) {
        match self.registers[IR] >> 12 { //Get opcode from 4 high bits

            //JnS
            0x0 => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = self.registers[PC];
                memory.write(self.registers[MAR], self.registers[MBR]);
                self.registers[PC] = self.registers[IR] + 1;
            },

            //LOAD
            0x1 => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[AC] = memory.read(self.registers[MAR])
            },

            //STORE
            0x2 => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = self.registers[AC];
                memory.write(self.registers[MAR], self.registers[MBR]);
            },

            //ADD
            0x3 => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] += self.registers[MBR];
            },

            //SUBT
            0x4 => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] -= self.registers[MBR];
            },

            //INPUT
            0x5 => {
                self.input();
            },

            //OUTPUT
            0x6 => {
                self.output();
            },

            //HALT
            0x7 => {
                std::process::exit(0);
            },

            //SKIPCOND
            0x8 => {
                let cond = self.registers[IR] & 0x0C00 >> 10; //Get condition from bits 11-10 and shift to LSB for easy comparison
                match cond {

                    0b00 => {
                        if self.registers[AC] < 0 {
                            self.registers[PC] += 1;
                        }
                    },

                    0x01 => {
                        if self.registers[AC] == 0 {
                            self.registers[PC] += 1;
                        }
                    },

                    0b10 => {
                        if self.registers[AC] > 0 {
                            self.registers[PC] += 1;
                        }
                    },
                    _ => {}
                }
            },

            //JUMP
            0x9 => {
                self.registers[PC] = self.registers[IR] & 0x0FFF;
            },

            //CLEAR
            0xA => {
                self.registers[AC] = 0x0000;
            },

            //ADDI
            0xB => {
                // read indirect
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] += self.registers[MBR];
            }

            //JUMPI
            0xC => {
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = self.registers[MAR];
                self.registers[PC] =self.registers[MBR];
            }

            //LOADI
            0xD => {
                // read indirect
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[AC] = self.registers[MBR];
            }

            //STOREI
            0xE => {
                // write indirect
                self.registers[MAR] = self.registers[IR] & 0x0FFF;
                self.registers[MBR] = memory.read(self.registers[MAR]);
                self.registers[MAR] = self.registers[MBR];
                self.registers[MBR] = self.registers[AC];
                memory.write(self.registers[MAR], self.registers[MBR]);
            },

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
        println!("Output: {}", self.registers[AC]);
    }

    fn run(&mut self, memory: &mut Memory) {
            loop {
                self.fetch(memory);
                self.decode_execute(memory);
            }
    }
    


}


struct Memory {
    mem: Vec<u16>
}

impl Memory {
    fn new(size: usize) -> Memory {
        Memory {
            mem: vec![0; size],
        }
    }

    fn read(&self, address:u16) -> u16 {
        self.mem[address as usize]   
    }

    fn write(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = value;
    }

    fn load_program(&mut self, program: Vec<u16>) {
        for (i, &instruction) in program.iter().enumerate() {
            self.write(i as u16, instruction);
        }
    }


}

enum Instruction {
    JnS(u16),
    Load(u16),
    Store(u16),
    Add(u16),
    Subt(u16),
    Input,
    Output,
    Halt
    Skipcond(u16),
    Jump(u16),
    Clear,
    AddI,
    JumpI,
    LoadI,
    StoreI
}


fn main() {
    let mut memory = Memory::new(4096);
    let program = vec![
        0x1100, // Load H (ASCII 72) into AC
        0x6100, // Output AC
        0x1101, // Load e (ASCII 101) into AC
        0x6100, // Output AC
        0x1102, // Load l (ASCII 108) into AC
        0x6100, // Output AC
        0x1102, // Load l (ASCII 108) into AC
        0x6100, // Output AC
        0x1103, // Load o (ASCII 111) into AC
        0x6100, // Output AC
        0x1104, // Load , (ASCII 44) into AC
        0x6100, // Output AC
        0x1105, // Load space (ASCII 32) into AC
        0x6100, // Output AC
        0x1106, // Load W (ASCII 87) into AC
        0x6100, // Output AC
        0x1107, // Load o (ASCII 111) into AC
        0x6100, // Output AC
        0x1108, // Load r (ASCII 114) into AC
        0x6100, // Output AC
        0x1102, // Load l (ASCII 108) into AC
        0x6100, // Output AC
        0x1109, // Load d (ASCII 100) into AC
        0x6100, // Output AC
        0x110A, // Load ! (ASCII 33) into AC
        0x6100, // Output AC
        0x7000, // Halt
        // Data section
        0x0048, // H (ASCII 72)
        0x0065, // e (ASCII 101)
        0x006C, // l (ASCII 108)
        0x006F, // o (ASCII 111)
        0x002C, // , (ASCII 44)
        0x0020, // space (ASCII 32)
        0x0057, // W (ASCII 87)
        0x0072, // r (ASCII 114)
        0x0064, // d (ASCII 100)
        0x0021, // ! (ASCII 33)
    ];
    memory.load_program(program);

    let mut cpu = CPU::new();
    cpu.run(&mut memory);
}