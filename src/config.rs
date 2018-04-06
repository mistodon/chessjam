pub use gen_config::{Config, CONFIG};
use std::borrow::Cow;

pub fn load_config() -> Cow<'static, Config> {
    #[cfg(debug_assertions)]
    {
        use std::fs::File;
        use std::io::Read;
        use toml;

        let mut file = File::open("assets/config.toml").unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        Cow::Owned(toml::from_str(&buffer).unwrap())
    }

    #[cfg(not(debug_assertions))]
    {
        Cow::Borrowed(&CONFIG)
    }
}
