# MARIE Simulator

This project is a Rust-based simulator for the MARIE (Machine Architecture that is Really Intuitive and Easy) model, an educational computer architecture designed to teach basic assembly language concepts and computer organization.

While this simulator handles the execution of MARIE programs, assembly is performed by a separate Python-based MARIE assembler. You can find the assembler project here: [assembler](https://github.com/falaventho/marie-assembler)

### How it works

At runtime, the simulator looks for a file named MARIE.ROM in the ROM directory. This ROM file is loaded into the memory space of the simulator, emulating the MARIE architecture.

### How to use it

1. Assemble a program using the [assembler](https://github.com/falaventho/marie-assembler). Or use a pre-assembled program (some are included with release).
2. Rename file to 'MARIE.ROM' to ensure the simulator reads it.
3. Place the file in the ROM directory.
4. Run the marie-simulator.exe using a terminal (add a ASCII command line argument to convert output into ASCII!)

### Requirements

- Windows 7 or newer
- Python3 (if using [assembler](https://github.com/falaventho/marie-assembler))

### License

This project is licensed under the MIT License.
