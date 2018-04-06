extern crate config_struct;

fn main() {
    let toml_config = config_struct::toml_parsing::parse_config_from_file("assets/config.toml").unwrap();
    config_struct::write_config_module("src/gen_config.rs", &toml_config, &Default::default()).unwrap();
}
