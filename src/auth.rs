use crate::config::Configuration;

#[derive(Debug)]
pub struct AuthManager {
    username: String,
    password: String,
    enabled: bool,
}

impl AuthManager {
    pub fn authorize(&self, username: &str, password: &str) -> bool {
        if !self.enabled {
            return true;
        }

        username == self.username && password == self.password
    }
}

impl From<&Configuration> for AuthManager {
    fn from(config: &Configuration) -> Self {
        Self {
            username: config.username.clone(),
            password: config.password.clone(),
            enabled: config.force_auth,
        }
    }
}
