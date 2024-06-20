use serde::Deserialize;

use rust_embed::Embed;

#[derive(Debug, Embed)]
#[folder = "src/resources/"]
#[include = "*.toml"]
pub struct Assert;

#[derive(Debug, Deserialize, Clone)]
pub struct SelectCharsPool {
    pub upper_chars_pool: String,
    pub lower_chars_pool: String,
    pub digital_chars_pool: String,
    pub mark_chars_pool: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigRead {
    pub select_chars_pool: SelectCharsPool,
}

impl Default for ConfigRead {
    fn default() -> Self {
        let config: ConfigRead;
        if let Some(file) = Assert::get("config.toml") {
            if let Ok(content) = std::str::from_utf8(file.data.as_ref()) {
                config = toml::from_str(content).unwrap();
                config
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

    #[test]
    fn test() {
        let config = crate::config_read::ConfigRead::get();
        println!("{:?}", config)
    }
}
