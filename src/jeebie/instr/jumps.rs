/// Module for 16 bit arithmetic (ALU instructions)
use jeebie::core::cpu::CPU;
use jeebie::registers::{ Flags, Register16 };

// 'JP nn' C3 12
pub fn JP_nn(cpu: &mut CPU) {
    let addr = cpu.get_immediate16();
    cpu.jump(addr);
}

// 'JP NZ,nn' C2 12
pub fn JP_NZ_nn(cpu: &mut CPU) {
    cpu.jump_not_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP Z,nn' CA 12
pub fn JP_Z_nn(cpu: &mut CPU) {
    cpu.jump_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP NC,nn' D2 12
pub fn JP_NC_nn(cpu: &mut CPU) {
    cpu.jump_not_flag(Flags::Carry, Register16::Immediate16);
}

// 'JP C,nn' DA 12
pub fn JP_C_nn(cpu: &mut CPU) {
    cpu.jump_flag(Flags::Zero, Register16::Immediate16);
}

// 'JP (HL)' E9 4
pub fn JP_HL(cpu: &mut CPU) {
    let addr = cpu.get16(Register16::HL);
    cpu.jump(addr);
}

// 'JR n' 18 8 
pub fn JR_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1; // it isn't updated yet
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16); // n is loaded as a signed bit
    cpu.jump(addr);
}

// 'JR NZ,*' 20 8
pub fn JR_NZ_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_not_flag(Flags::Zero, Register16::Value16(addr));
}

// 'JR Z,*' 28 8
pub fn JR_Z_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_flag(Flags::Zero, Register16::Value16(addr));
}

// 'JR NC,*' 30 8
pub fn JR_NC_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_not_flag(Flags::Carry, Register16::Value16(addr));
}

// 'JR C,*' 38 8
pub fn JR_C_n(cpu: &mut CPU) {
    let pc = cpu.reg.pc + 1;
    let n = cpu.mem.read_b(cpu.reg.pc);
    let addr = pc.wrapping_add((n as i8) as u16);
    cpu.jump_flag(Flags::Carry, Register16::Value16(addr));
}

// 'CALL nn' CD 12
pub fn CALL_nn(cpu: &mut CPU) {
    let call_addr = cpu.get16(Register16::Immediate16);
    let next_instr = cpu.reg.pc.wrapping_add(1);
    cpu.push_stack(Register16::Value16(next_instr));
    cpu.jump(call_addr);
}

// 'CALL NZ,nn' C4 12
pub fn CALL_NZ_nn(cpu: &mut CPU) {
    if !cpu.reg.is_set(Flags::Zero) {
        CALL_nn(cpu);
    }
}

// 'CALL Z,nn' CC 12
pub fn CALL_Z_nn(cpu: &mut CPU) {
    if cpu.reg.is_set(Flags::Zero) {
        CALL_nn(cpu);
    }
}

// 'CALL NC,nn' D4 12
pub fn CALL_NC_nn(cpu: &mut CPU) {
    if !cpu.reg.is_set(Flags::Carry) {
        CALL_nn(cpu);
    }
}

// 'CALL C,nn' DC 12
pub fn CALL_C_nn(cpu: &mut CPU) {
    if cpu.reg.is_set(Flags::Carry) {
        CALL_nn(cpu);
    }
}

// 'RST 00H' C7 32
pub fn RST_00h(cpu: &mut CPU) {
    cpu.restart(0x00);
}

// 'RST 08H' CF 32
pub fn RST_08h(cpu: &mut CPU) {
    cpu.restart(0x08);
}

// 'RST 10H' D7 32
pub fn RST_10h(cpu: &mut CPU) {
    cpu.restart(0x10);
}

// 'RST 18H' DF 32
pub fn RST_18h(cpu: &mut CPU) {
    cpu.restart(0x18);
}

// 'RST 20H' E7 32
pub fn RST_20h(cpu: &mut CPU) {
    cpu.restart(0x20);
}

// 'RST 28H' EF 32
pub fn RST_28h(cpu: &mut CPU) {
    cpu.restart(0x28);
}

// 'RST 30H' F7 32
pub fn RST_30h(cpu: &mut CPU) {
    cpu.restart(0x30);
}

// 'RST 38H' FF 32
pub fn RST_38h(cpu: &mut CPU) {
    cpu.restart(0x38);
}