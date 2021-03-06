mod apu_impl;
mod dmc;
mod envelope;
mod frame_counter;
mod length;
mod noise;
mod pulse;
mod timer;
mod triangle;

use crate::bitops::BitOps;
use dmc::DMC;
use frame_counter::FrameCounter;
use noise::Noise;
use pulse::{Negation, Pulse};
use triangle::Triangle;

pub trait APU<'a> {
    fn apu_cycle(&mut self);
    fn get_apu_buffer(&'a self) -> &'a [f32];
    fn clear_apu_buffer(&mut self);
}

/// Represents the APU's internal state
pub struct APUState {
    buffer: Vec<f32>,
    pub pulse1: Pulse,
    pub pulse2: Pulse,
    pub triangle: Triangle,
    pub noise: Noise,
    pub dmc: DMC,
    pub frame_counter: FrameCounter,
}

impl APUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        APUState {
            buffer: Vec::new(),
            pulse1: Pulse::new(Negation::OnesComplement),
            pulse2: Pulse::new(Negation::TwosComplement),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: DMC::new(),
            frame_counter: FrameCounter::new(),
        }
    }

    pub fn set_status(&mut self, val: u8) {
        if !val.is_bit_set(4) {
            self.dmc.cur_length = 0;
        } else if self.dmc.cur_length == 0 {
            self.dmc.cur_length = self.dmc.sample_length;
            self.dmc.cur_addr = self.dmc.sample_addr;
        }
        self.dmc.irq_pending = false;

        self.noise.length_counter.silent(val.is_bit_set(3));
        self.triangle.length_counter.silent(val.is_bit_set(2));
        self.pulse2.length_counter.silent(val.is_bit_set(1));
        self.pulse1.length_counter.silent(val.is_bit_set(0));
    }

    pub fn get_status(&self) -> u8 {
        let mut result = 0;
        result.assign_bit(7, self.dmc.irq_pending);
        result.assign_bit(6, self.frame_counter.get_irq_pending());
        result.assign_bit(4, self.dmc.cur_length > 0);
        result.assign_bit(3, !self.noise.length_counter.is_zero());
        result.assign_bit(2, !self.triangle.length_counter.is_zero());
        result.assign_bit(1, !self.pulse2.length_counter.is_zero());
        result.assign_bit(0, !self.pulse1.length_counter.is_zero());
        result
    }

    pub fn set_frame_counter(&mut self, val: u8) {
        self.frame_counter.set(val);
        if val.is_bit_set(7) {
            self.quarter_frame();
            self.half_frame();
        }
    }

    pub fn quarter_frame(&mut self) {
        self.pulse1.envelope.clock();
        self.pulse2.envelope.clock();
        self.triangle.decrement_linear();
        self.noise.envelope.clock();
    }

    pub fn half_frame(&mut self) {
        self.pulse1.sweep();
        self.pulse1.length_counter.decrement();
        self.pulse2.sweep();
        self.pulse2.length_counter.decrement();
        self.triangle.length_counter.decrement();
        self.noise.length_counter.decrement();
    }
}
