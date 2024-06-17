use serde::Deserialize;

use rust_embed::Embed;

#[derive(Debug, Embed)]
#[folder = "src/resources/"]
#[include = "*.toml"]
pub struct Assert;

#[derive(Debug, Deserialize)]
pub struct SelectCharsPool {
    pub upper_chars_pool: String,
    pub lower_chars_pool: String,
    pub digital_chars_pool: String,
    pub mark_chars_pool: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRead {
    pub select_chars_pool: SelectCharsPool,
}

impl Default for ConfigRead {
    fn default() -> Self {
        let config_e: ConfigRead;
        if let Some(e_file) = Assert::get("config.toml") {
            if let Ok(s) = std::str::from_utf8(e_file.data.as_ref()) {
                config_e = toml::from_str(s).unwrap();
                return config_e;
            } else {
                panic!("没有找到配置文件");
            }
        } else {
            panic!("没有找到配置文件");
        }
    }
}

impl ConfigRead {
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: ConfigRead = ConfigRead::default();
        }
        &CACHE
    }
}

mod test_config {
    use super::*;
    #[test]
    fn test() {
        let config = ConfigRead::get();
        println!("{:?}", config)
    }
}
