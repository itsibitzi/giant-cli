use std::error::Error;

use reqwest::{self, Url};

pub struct GiantApi {
    uri: String,
}

impl GiantApi {
    pub fn new(uri: String) -> GiantApi {
        GiantApi { uri }
    }

    pub async fn login(&self, token: &str) -> Result<(), Box<dyn Error>> {
        let url: Url = (self.uri.clone() + "/api/keepalive").parse()?;

        let client = reqwest::Client::new();
        let res = client.get(url).bearer_auth(token).send().await?;

        let headers = res.headers();

        match (
            headers.get("X-Offer-Authorization"),
            headers.get("WWW-Authenticate"),
        ) {
            (Some(auth), None) => {
                // Save to credeital
            }
            _ => {}
        }

        Ok(())
    }
}
