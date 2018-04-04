#![allow(unknown_lints)] // TODO(***realname***): Can go when clippy lints go.

use std::time::Instant;

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
