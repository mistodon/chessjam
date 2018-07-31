extern crate config_struct;

fn main() {
    use config_struct::{Options, FloatSize, IntSize};

    println!("cargo:rerun-if-changed=src");

    config_struct::create_config(
        "assets/config.toml",
        "src/gen_config.rs",
        &Options {
            default_float_size: FloatSize::F32,
            default_int_size: IntSize::I32,
            max_array_size: 4,
            ..Default::default()
        }).unwrap();
}
