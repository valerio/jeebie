use gbe::cpu::CPU;

// LD B,n 06 8
pub fn LD_B_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.bc.high, cpu.reg.pc.low.get());
}

// LD C,n 0E 8
pub fn LD_C_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.bc.low, cpu.reg.pc.low.get());
}

// LD D,n 16 8
pub fn LD_D_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.de.high, cpu.reg.pc.low.get());
}

// LD E,n 1E 8
pub fn LD_E_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.de.low, cpu.reg.pc.low.get());
}

// LD H,n 26 8
pub fn LD_H_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.hl.high, cpu.reg.pc.low.get());
}

// LD L,n 2E 8
pub fn LD_L_n(cpu: &mut CPU) {
    CPU::LD_nn_n(&mut cpu.reg.hl.low, cpu.reg.pc.low.get());
}

// LD A,A 7F 4
pub fn LD_a_a(cpu: &mut CPU) {
    let v = cpu.reg.af.high.get();
    cpu.reg.af.high.set(v);
}

// LD A,B 78 4
pub fn LD_a_b(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.bc.high.get());
}

// LD A,C 79 4
pub fn LD_a_c(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.bc.low.get());
}

// LD A,D 7A 4
pub fn LD_a_d(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.de.high.get());
}

// LD A,E 7B 4
pub fn LD_a_e(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.de.low.get());
}

// LD A,H 7C 4
pub fn LD_a_h(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.hl.high.get());
}

// LD A,L 7D 4
pub fn LD_a_l(cpu: &mut CPU) {
    cpu.reg.af.high.set(cpu.reg.hl.low.get());
}

// LD B,B 40 4
pub fn LD_b_b(cpu: &mut CPU) {
    let v = cpu.reg.bc.high.get();
    cpu.reg.bc.high.set(v);
}
// LD B,C 41 4
pub fn LD_b_c(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.bc.low.get());
}

// LD B,D 42 4
pub fn LD_b_d(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.de.high.get());
}

// LD B,E 43 4
pub fn LD_b_e(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.de.low.get());
}

// LD B,H 44 4
pub fn LD_b_h(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.hl.high.get());
}

// LD B,L 45 4
pub fn LD_b_l(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.hl.low.get());
}

// LD B,A 47 4
pub fn LD_b_a(cpu: &mut CPU) {
    cpu.reg.bc.high.set(cpu.reg.af.high.get());
}

// LD B,(HL) 46 8
pub fn LD_b_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.bc.high.set(m.read_b(cpu.reg.hl.get()));
}

// LD C,B 48 4
pub fn LD_c_b(cpu: &mut CPU) {
    let v = cpu.reg.bc.high.get();
    cpu.reg.bc.low.set(v);
}

// LD C,C 49 4
pub fn LD_c_c(cpu: &mut CPU) {
    let v = cpu.reg.bc.low.get();
    cpu.reg.bc.low.set(v);
}

// LD C,D 4A 4
pub fn LD_c_d(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.de.high.get());
}

// LD C,E 4B 4
pub fn LD_c_e(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.de.low.get());
}

// LD C,H 4C 4
pub fn LD_c_h(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.hl.high.get());
}

// LD C,L 4D 4
pub fn LD_c_l(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.hl.low.get());
}

// LD C,A 4F 4
pub fn LD_c_a(cpu: &mut CPU) {
    cpu.reg.bc.low.set(cpu.reg.af.high.get());
}

// LD C,(HL) 4E 8
pub fn LD_c_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.bc.low.set(m.read_b(cpu.reg.hl.get()));
}

// LD D,B 50 4
pub fn LD_d_b(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.bc.high.get());
}

// LD D,C 51 4
pub fn LD_d_c(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.bc.low.get());
}

// LD D,D 52 4
pub fn LD_d_e(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.de.low.get());
}

// LD D,E 53 4
pub fn LD_d_d(cpu: &mut CPU) {
    let v = cpu.reg.de.high.get();
    cpu.reg.de.high.set(v);
}

// LD D,H 54 4
pub fn LD_d_h(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.hl.high.get());
}

// LD D,L 55 4
pub fn LD_d_l(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.hl.low.get());
}

// LD D,A 57 4
pub fn LD_d_a(cpu: &mut CPU) {
    cpu.reg.de.high.set(cpu.reg.af.high.get());
}

// LD D,(HL) 56 8
pub fn LD_d_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.de.high.set(m.read_b(cpu.reg.hl.get()));
}

// LD E,B 58 4
pub fn LD_e_b(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.bc.high.get());
}

// LD E,C 59 4
pub fn LD_e_c(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.bc.low.get());
}

// LD E,D 5A 4
pub fn LD_e_d(cpu: &mut CPU) {
    let v = cpu.reg.de.high.get();
    cpu.reg.de.low.set(v);
}

// LD E,E 5B 4
pub fn LD_e_e(cpu: &mut CPU) {
    let v = cpu.reg.de.low.get();
    cpu.reg.de.low.set(v);
}

// LD E,H 5C 4
pub fn LD_e_h(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.hl.high.get());
}

// LD E,L 5D 4
pub fn LD_e_l(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.hl.low.get());
}

// LD E,(HL) 5E 8
pub fn LD_e_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.de.low.set(m.read_b(cpu.reg.hl.get()));
}

// LD E,A 5F 4
pub fn LD_e_a(cpu: &mut CPU) {
    cpu.reg.de.low.set(cpu.reg.af.high.get());
}

// LD H,B 60 4
pub fn LD_h_b(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.bc.high.get());
}

// LD H,C 61 4
pub fn LD_h_c(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.bc.low.get());
}

// LD H,D 62 4
pub fn LD_h_d(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.de.high.get());
}

// LD H,E 63 4
pub fn LD_h_e(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.de.low.get());
}

// LD H,H 64 4
pub fn LD_h_h(cpu: &mut CPU) {
    let v = cpu.reg.hl.high.get();
    cpu.reg.hl.high.set(v);
}

// LD H,L 65 4
pub fn LD_h_l(cpu: &mut CPU) {
    let v = cpu.reg.hl.low.get();
    cpu.reg.hl.high.set(v);
}

// LD H,(HL) 66 8
pub fn LD_h_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    let value = m.read_b(cpu.reg.hl.get());
    cpu.reg.hl.high.set(value);
}

// LD H,A 67 4
pub fn LD_h_a(cpu: &mut CPU) {
    cpu.reg.hl.high.set(cpu.reg.af.high.get());
}

// LD L,B 68 4
pub fn LD_l_b(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.bc.high.get());
}

// LD L,C 69 4
pub fn LD_l_c(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.bc.low.get());
}

// LD L,D 6A 4
pub fn LD_l_d(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.de.high.get());
}

// LD L,E 6B 4
pub fn LD_l_e(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.de.low.get());
}

// LD L,H 6C 4
pub fn LD_l_h(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.hl.high.get());
}

// LD L,L 6D 4
pub fn LD_l_l(cpu: &mut CPU) {
    let v = cpu.reg.hl.low.get();
    cpu.reg.hl.low.set(v);
}

// LD L,A 6F 4
pub fn LD_l_a(cpu: &mut CPU) {
    cpu.reg.hl.low.set(cpu.reg.af.high.get());
}

// LD L,(HL) 6E 8
pub fn LD_l_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    let value = m.read_b(cpu.reg.hl.get());
    cpu.reg.hl.low.set(value);
}

// LD (HL),B 70 8
pub fn LD_HLm_b(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.bc.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),C 71 8
pub fn LD_HLm_c(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.bc.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),D 72 8
pub fn LD_HLm_d(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.de.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),E 73 8
pub fn LD_HLm_e(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.de.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),H 74 8
pub fn LD_HLm_h(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.hl.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),L 75 8
pub fn LD_HLm_l(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.hl.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),n 36 12
pub fn LD_HLm_n(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.pc.low.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD (HL),A 77 8
pub fn LD_HLm_a(cpu: &mut CPU) {
    let addr = cpu.reg.hl.get();
    let data = cpu.reg.af.high.get();
    let mut m = cpu.mem.borrow_mut();
    m.write_b(addr, data);
}

// LD A,(BC) 0A 8
pub fn LD_a_BCm(cpu: &mut CPU) {
    let addr = cpu.reg.bc.get();
    let data = cpu.mem.borrow().read_b(addr);
    cpu.reg.af.high.set(data);
}

// LD A,(DE) 1A 8
pub fn LD_a_DEm(cpu: &mut CPU) {
    let addr = cpu.reg.de.get();
    let data = cpu.mem.borrow().read_b(addr);
    cpu.reg.af.high.set(data);
}

// LD A,(HL) 7E 8
pub fn LD_a_HLm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    cpu.reg.af.high.set(m.read_b(cpu.reg.hl.get()));
}

// LD A,(nn) FA 16
pub fn LD_a_nnm(cpu: &mut CPU) {
    let m = cpu.mem.borrow();
    // immediate value (pc) is used as address
    let addr = cpu.reg.pc.get();
    cpu.reg.af.high.set(m.read_b(addr));
}

// LD A,# 3E 8
// TODO: implement this?

// LD (BC),A 02 8
pub fn LD_BCm_A(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let addr = cpu.reg.bc.get();
    cpu.mem.borrow_mut().write_b(addr, value);
}

// LD (DE),A 12 8
pub fn LD_DEm_A(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let addr = cpu.reg.de.get();
    cpu.mem.borrow_mut().write_b(addr, value);
}

// LD (nn),A EA 16
pub fn LD_nnm_A(cpu: &mut CPU) {
    let value = cpu.reg.af.high.get();
    let addr = cpu.reg.pc.get();
    cpu.mem.borrow_mut().write_b(addr, value);
}

// LD A,(C) F2 8
// Put value at address $FF00 + register C into A.
// Same as: LD A,($FF00+C)
pub fn LD_a_c_mem(cpu: &mut CPU) {
    let addr = 0xFF00 & (cpu.reg.bc.low.get() as u16);
    let value = cpu.mem.borrow().read_b(addr);
    cpu.reg.af.high.set(value);
}

// LD ($FF00+C),A E2 8
// Put value of A at address $FF00 + register C.
pub fn LD_c_mem_a(cpu: &mut CPU) {
    let addr = 0xFF00 & (cpu.reg.bc.low.get() as u16);
    let value = cpu.reg.af.high.get();
    cpu.mem.borrow_mut().write_b(addr, value);
}