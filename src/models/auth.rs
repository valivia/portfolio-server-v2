use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub test: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubWebhook {
    pub test: String,
}

pub struct LastLogin {
    pub code: Arc<Mutex<String>>,
}
