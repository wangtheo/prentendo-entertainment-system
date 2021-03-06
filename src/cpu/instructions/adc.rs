use super::{Instruction, InstructionName, Read};
use crate::bitops::BitOps;
use crate::cpu::variables::Flag;
use crate::state::CPU;

/// Represents the ADC instruction (http://www.obelisk.me.uk/6502/reference.html#ADC)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ADC;

impl Instruction for ADC {
    fn name(&self) -> InstructionName {
        InstructionName::ADC
    }
}

impl<S: CPU> Read<S> for ADC {
    fn execute(&self, cpu: &mut S, addr: u16) {
        let byte: u8 = cpu.get_mem(addr);
        let c: u8 = if cpu.is_flag_set(Flag::C) { 1 } else { 0 };
        let a: u8 = cpu.get_a();
        let (result, overflow1): (u8, bool) = a.overflowing_add(byte);
        let (result, overflow2): (u8, bool) = result.overflowing_add(c);
        cpu.assign_flag(Flag::N, result.is_bit_set(7));
        cpu.assign_flag(Flag::Z, result == 0);
        cpu.assign_flag(Flag::C, overflow1 || overflow2);
        // if result's sign is opposite to a and byte has the same sign as a
        cpu.assign_flag(Flag::V, ((result ^ a) & !(byte ^ a)).is_bit_set(7));
        cpu.set_a(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::cpu::{Memory, Registers};
    use crate::state::NES;

    #[test]
    fn test_adc() {
        let mut cpu = NES::mock();
        cpu.set_p(0b0010_0000);
        cpu.assign_flag(Flag::C, true);
        cpu.set_a(132);
        cpu.set_mem(cpu.get_pc(), 40);
        let addr: u16 = cpu.get_pc();
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.get_a(), 173);
    }

    #[test]
    fn test_adc_n() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::N, false);
        cpu.set_a(0b0100_0000);
        cpu.set_mem(cpu.get_pc(), 0b1000_0000);
        let addr: u16 = cpu.get_pc();
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::N), true);
        cpu.assign_flag(Flag::N, false);
        cpu.set_mem(addr, 0b0100_0000);
        cpu.set_a(0b0010_0000);
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::N), false);
    }

    #[test]
    fn test_adc_z() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::Z, false);
        cpu.set_a(0b0100_0000);
        cpu.set_mem(cpu.get_pc(), 0b1000_0000);
        let addr: u16 = cpu.get_pc();
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::Z), false);
        cpu.assign_flag(Flag::Z, false);
        cpu.set_mem(addr, 0);
        cpu.set_a(0);
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::Z), true);
    }

    #[test]
    fn test_adc_c() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::C, false);
        cpu.set_p(0b0010_0000);
        cpu.set_a(0b1111_1111);
        cpu.set_mem(cpu.get_pc(), 0b1000_0000);
        let addr: u16 = cpu.get_pc();
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        cpu.assign_flag(Flag::C, false);
        cpu.set_mem(addr, 0b0100_0000);
        cpu.set_a(0b0010_0000);
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::C), false);
    }

    #[test]
    fn test_adc_v() {
        let mut cpu = NES::mock();
        cpu.assign_flag(Flag::V, false);
        cpu.set_a(64i8 as u8);
        cpu.set_mem(cpu.get_pc(), 72i8 as u8);
        let addr: u16 = cpu.get_pc();
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::V), true);
        cpu.assign_flag(Flag::V, false);
        cpu.set_mem(addr, 4i8 as u8);
        cpu.set_a(43i8 as u8);
        ADC.execute(&mut cpu, addr);
        assert_eq!(cpu.is_flag_set(Flag::V), false);
    }
}
