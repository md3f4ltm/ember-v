mod cpu;
use cpu::Cpu;

fn main() -> Result<(), &'static str> {
    let mut cpu = Cpu::new();
    let program = [
        0x02a0_0093, // addi x1, x0, 42
        0xfff0_8113, // addi x2, x1, -1
        0x0020_81b3, // add  x3, x1, x2
    ];

    cpu.step(&program)?; // addi x1, x0, 42

    assert_eq!(cpu.registers[1], 42);
    assert_eq!(cpu.registers[0], 0);
    assert_eq!(cpu.pc, 4);

    println!("x1 = {}, pc = {}", cpu.registers[1], cpu.pc);
    Ok(())
}
