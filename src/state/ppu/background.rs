use super::Background;
use crate::state::NES;

impl Background for NES {
    fn get_nametable_addr(&self) -> u16 {
        0x2000 | (self.ppu.internal_registers.v.get() & 0x0FFF)
    }
    fn get_attribute_addr(&self) -> u16 {
        0x23C0
            | (self.ppu.internal_registers.v.get() & 0x0C00)
            | ((self.ppu.internal_registers.v.get() >> 4) & 0x38)
            | ((self.ppu.internal_registers.v.get() >> 2) & 0x07)
    }
    fn get_background_tile_addr_low(&self, index: u8) -> u16 {
        self.ppu
            .mapped_registers
            .get_background_pattern_table_addr()
            + ((index as u16) << 4)
            + self.ppu.internal_registers.fine_y()
    }
    fn get_background_tile_addr_high(&self, index: u8) -> u16 {
        self.ppu
            .mapped_registers
            .get_background_pattern_table_addr()
            + ((index as u16) << 4)
            + 0b1000
            + self.ppu.internal_registers.fine_y()
    }
}