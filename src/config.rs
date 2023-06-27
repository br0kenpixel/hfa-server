use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    #[serde(rename = "max-upload-size")]
    pub max_upload_size: u64,
    #[serde(rename = "allow-index")]
    pub allow_index: bool,
    #[serde(rename = "blocked-methods")]
    pub blocked_methods: Vec<Method>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Method {
    List,
    Get,
    Put,
    Update,
    Delete,
}

impl Configuration {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
