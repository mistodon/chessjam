#![allow(unknown_lints)] // TODO(***realname***): Can go when clippy lints go.

#[macro_use]
extern crate serde_derive;

#[cfg(debug_assertions)]
extern crate toml;

extern crate adequate_math;

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

pub fn world_to_grid(point: Vec3<f32>) -> Vec2<i32> {
    let (x, _, y) = point.as_tuple();
    (vec2(x, y) + vec2(3.5, 3.5)).map(f32::round).as_i32()
}

pub fn grid_to_world(point: Vec2<i32>) -> Vec3<f32> {
    let (x, y) = point.as_tuple();
    vec3(x, 0, y).as_f32() - vec3(3.5, 0.0, 3.5)
}
