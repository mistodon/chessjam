#![allow(unknown_lints)] // TODO(claire): Can go when clippy lints go.

#[macro_use]
extern crate serde_derive;

#[cfg(debug_assertions)]
extern crate toml;

extern crate adequate_math;
extern crate image;

pub mod config;
mod gen_config;

use std::time::Instant;

use adequate_math::*;


pub fn delta_time(previous_time: Instant) -> (f32, Instant) {
    let now = Instant::now();
    let delta = now.duration_since(previous_time);
    let dt =
        (delta.as_secs() as f32) + (delta.subsec_nanos() as f32 / 1_000_000_000.0);
    (dt, now)
}

pub fn elapsed_time(start_time: Instant) -> f32 {
    let (dt, _) = delta_time(start_time);
    dt
}

pub fn viewport_rect(
    screen_dimensions: (u32, u32),
    target_aspect: f32,
) -> (u32, u32, u32, u32) {
    let (pixel_width, pixel_height) = screen_dimensions;
    let (screen_width, screen_height) = (pixel_width as f32, pixel_height as f32);

    let actual_aspect = screen_width / screen_height;
    let error = actual_aspect / target_aspect;

    let width = (screen_width / error.max(1.0)) as u32;
    let height = (screen_height * error.min(1.0)) as u32;
    let left = (pixel_width - width) / 2;
    let bottom = (pixel_height - height) / 2;

    (left, bottom, width, height)
}

pub fn viewport_stretch(
    screen_dimensions: (u32, u32),
    viewport_width: u32,
    viewport_height: u32,
) -> Vec2<f32> {
    let (w, h) = screen_dimensions;

    vec2(
        w as f32 / viewport_width as f32,
        h as f32 / viewport_height as f32,
    )
}

pub fn world_to_grid(point: Vec3<f32>) -> Vec2<i32> {
    let (x, _, y) = point.as_tuple();
    (vec2(x, y) + vec2(3.5, 3.5))
        .map(f32::round)
        .as_i32()
}

pub fn grid_to_world(point: Vec2<i32>) -> Vec3<f32> {
    let (x, y) = point.as_tuple();
    vec3(x, 0, y).as_f32() - vec3(3.5, 0.0, 3.5)
}

pub fn grid_from_u8(square: u8) -> Vec2<i32> {
    vec2(square % 8, square / 8).as_i32()
}

pub fn valid_square(tile: Vec2<i32>) -> bool {
    let (x, y) = tile.as_tuple();
    x >= 0 && x < 8 && y >= 0 && y < 8
}

pub fn decode_image(bytes: &[u8]) -> (Vec<u8>, (u32, u32)) {
    use image::{self, ImageFormat};

    let image = image::load_from_memory_with_format(bytes, ImageFormat::PNG)
        .unwrap()
        .to_rgba();
    let image_dimensions = image.dimensions();
    let raw_bytes = image.into_raw();
    (raw_bytes, image_dimensions)
}
