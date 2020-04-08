use super::{Implied, Instruction, InstructionName};
use crate::state::CPU;
use crate::cpu::variables::Flag;

/// Represents the 'set flag' instructions
/// (http://www.obelisk.me.uk/6502/reference.html#SEC)
/// (http://www.obelisk.me.uk/6502/reference.html#SED)
/// (http://www.obelisk.me.uk/6502/reference.html#SEI)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SE(pub Flag);

impl Instruction for SE {
    fn name(&self) -> InstructionName {
        InstructionName::SE(self.0)
    }
}

impl<S: CPU> Implied<S> for SE {
    fn execute(&self, cpu: &mut S) {
        cpu.set_flag(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NES;
    use crate::state::cpu::Registers;

    #[test]
    fn test_sec() {
        let mut cpu = NES::mock();
        cpu.clear_flag(Flag::C);
        SE(Flag::C).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
        SE(Flag::Z).execute(&mut cpu);
        assert_eq!(cpu.is_flag_set(Flag::C), true);
    }
}
