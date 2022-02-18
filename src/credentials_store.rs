use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Deserialize, Serialize)]
pub struct CredentialsStore {
    instanceCredentials: HashMap<String, InstanceCredentials>,
}

#[derive(Deserialize, Serialize)]
pub struct InstanceCredentials {
    token: String,
    saved_at: DateTime<Utc>,
}

impl CredentialsStore {
    fn new() -> CredentialsStore {
        CredentialsStore {
            instanceCredentials: HashMap::new(),
        }
    }

    pub fn from_file(file_path: impl AsRef<Path>) -> Result<CredentialsStore, Box<dyn Error>> {
        if file_path.as_ref().exists() {
            let text = fs::read_to_string(file_path)?;
            Ok(serde_json::from_str::<CredentialsStore>(&text)?)
        } else {
            Ok(CredentialsStore::new())
        }
    }
}
