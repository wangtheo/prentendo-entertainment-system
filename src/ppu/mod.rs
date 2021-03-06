mod background_evaluation;
pub mod display;
mod palette;
mod pipeline;
mod sprite_evaluation;

use crate::state::PPU;
use background_evaluation::evaluate_background;
use palette::NES_COLORS;
use pipeline::Pipeline;
use sprite_evaluation::evaluate_sprites;
use std::cell::RefCell;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

/// This function decides what happens on each cycle, yielding pixels along the way
pub fn cycle<'a, T: PPU>(
    ppu: &'a RefCell<T>,
) -> impl Generator<Yield = Option<Pixel>, Return = ()> + 'a {
    move || {
        let mut pipeline = Pipeline::new();
        let mut background_generator = evaluate_background(ppu);
        let mut sprite_generator = evaluate_sprites(ppu);
        loop {
            let scanline: usize = ppu.borrow().get_scanline();
            let tick: usize = ppu.borrow().get_tick();
            let background_enabled: bool = ppu.borrow().should_render_background();
            let sprites_enabled: bool = ppu.borrow().should_render_sprites();

            // the zero tick is always idle - nothing happens other than a cycle update
            if tick == 0 {
                ppu.borrow_mut().update_cycle();
                yield None;
                continue;
            }

            let mut pixel: Option<Pixel> = None;
            if should_output_pixel(scanline, tick) && (background_enabled || sprites_enabled) {
                let fine_x: u8 = ppu.borrow().get_fine_x();
                if let Some((addr, sprite0)) = pipeline.get_next_palette_addr(fine_x) {
                    let color: Color = NES_COLORS[usize::from(ppu.borrow().get(addr))];
                    if sprite0 && background_enabled && sprites_enabled && tick != 256 {
                        ppu.borrow_mut().trigger_sprite_zero();
                    }
                    pixel = Some(Pixel {
                        x: tick - 1,
                        y: scanline,
                        color,
                    });
                }
            }
            yield pixel;

            if should_reload_background(scanline, tick) {
                background_generator = evaluate_background(ppu);
            }

            if should_reload_sprites(scanline, tick) {
                sprite_generator = evaluate_sprites(ppu);
            }

            if should_run_background(scanline, tick) {
                pipeline.advance_background();
            }

            if should_run_background(scanline, tick) && background_enabled {
                match Pin::new(&mut background_generator).resume(()) {
                    GeneratorState::Complete(tile) => {
                        pipeline.load_background_tile(tile);
                        ppu.borrow_mut().increment_x();
                        background_generator = evaluate_background(ppu);
                    }
                    _ => {}
                }
            }

            if should_run_sprites(scanline, tick) {
                pipeline.advance_sprites();
            }

            if should_run_sprites(scanline, tick) && (background_enabled || sprites_enabled) {
                match Pin::new(&mut sprite_generator).resume(()) {
                    GeneratorState::Complete(sprites) => {
                        pipeline.load_sprites(sprites);
                    }
                    _ => {}
                }
            }

            if should_increment_y(scanline, tick) && (background_enabled || sprites_enabled) {
                ppu.borrow_mut().increment_y();
            }

            if should_reset_x(scanline, tick) && (background_enabled || sprites_enabled) {
                ppu.borrow_mut().reset_x();
            }

            if should_reset_y(scanline, tick) && (background_enabled || sprites_enabled) {
                ppu.borrow_mut().reset_y();
            }

            if should_set_vblank(scanline, tick) {
                ppu.borrow_mut().start_vblank();
            }

            if should_clear_flags(scanline, tick) {
                ppu.borrow_mut().end_vlbank();
                ppu.borrow_mut().clear_sprite_zero();
                ppu.borrow_mut().clear_sprite_overflow();
            }

            ppu.borrow_mut().update_cycle();
        }
    }
}

fn on_render_line(scanline: usize) -> bool {
    (0..=239).contains(&scanline) || scanline == 261
}

fn should_output_pixel(scanline: usize, tick: usize) -> bool {
    (1..=256).contains(&tick) && (0..=239).contains(&scanline)
}

fn should_run_background(scanline: usize, tick: usize) -> bool {
    ((1..=256).contains(&tick) || (321..=336).contains(&tick))
        && on_render_line(scanline)
        && tick != 0
}

fn should_run_sprites(scanline: usize, tick: usize) -> bool {
    (0..=239).contains(&scanline) && tick != 0
}

fn should_reload_sprites(scanline: usize, tick: usize) -> bool {
    should_run_sprites(scanline, tick) && tick == 1
}

fn should_reload_background(scanline: usize, tick: usize) -> bool {
    should_run_background(scanline, tick) && tick % 8 == 1
}

fn should_increment_y(scanline: usize, tick: usize) -> bool {
    tick == 256 && on_render_line(scanline)
}

fn should_reset_x(scanline: usize, tick: usize) -> bool {
    tick == 257 && on_render_line(scanline)
}

fn should_reset_y(scanline: usize, tick: usize) -> bool {
    (280..=304).contains(&tick) && scanline == 261
}

fn should_set_vblank(scanline: usize, tick: usize) -> bool {
    scanline == 241 && tick == 1
}

fn should_clear_flags(scanline: usize, tick: usize) -> bool {
    scanline == 261 && tick == 1
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
