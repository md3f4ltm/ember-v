pub struct Cpu {
    pub registers: [u32; 32],
    pub pc: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
        }
    }

    pub fn execute(&mut self, instruction: u32) -> Result<(), &'static str> {
        let opcode = instruction & 0x7f;
        let funct3 = (instruction >> 12) & 0x7;

        if opcode != 0x13 || funct3 != 0 {
            return Err("My bad bro, only addi for now");
        }

        let rd = ((instruction >> 7) & 0x1f) as usize;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let imm = (instruction as i32) >> 20;

        if rd != 0 {
            self.registers[rd] = self.registers[rs1].wrapping_add(imm as u32);
        }

        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    pub fn step(&mut self, program: &[u32]) -> Result<(), &'static str> {
        let index = (self.pc / 4) as usize;
        let instruction = *program.get(index).ok_or("PC is outside of program")?;
        self.execute(instruction)
    }
}
