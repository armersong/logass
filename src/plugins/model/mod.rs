use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ColorConfig {
    pub log: Vec<ColorLogConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ColorLogConfig {
    #[serde(rename = "match")]
    pub match_: String,
    pub level: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IgnoreConfig {
    pub rules: Vec<String>,
}
