#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CycleStatus {
    pub cycle: usize,
    pub scanline: usize,
    pub is_odd_frame: bool,
}

impl CycleStatus {
    pub const MAX_SCANLINES: usize = 262;
    pub const MAX_CYCLES: usize = 341;

    pub fn new() -> Self {
        CycleStatus {
            cycle: 0,
            scanline: 0,
            is_odd_frame: false,
        }
    }

    pub fn is_on_render_line(&self) -> bool {
        (0..=239).contains(&self.scanline) || self.scanline == 261
    }
}
