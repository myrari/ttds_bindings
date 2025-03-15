use std::collections::HashMap;

use reqwest::{Client, Method, Request};
use url::Url;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Pane {
    pub name: String,
    pub bg_color: Color,
    //pub conn: Connection,
    token: Option<String>,
}

impl Pane {
    pub fn get_token(&self) -> Result<String, String> {
        match &self.token {
            Some(token) => Ok(token.clone()),
            None => Err(String::from(
                "Token not yet initialized! Did you succesfully open a connection?",
            )),
        }
    }
}

pub struct Connection {
    host: String,
    use_tls: bool,
}

impl Connection {
    pub fn new(host: impl Into<String>, use_tls: bool) -> Self {
        Self {
            host: host.into(),
            use_tls,
        }
    }

    pub async fn request(
        self,
        args: &[String], // TODO: Find better name for this
        method: Option<Method>,
        auth: Option<&str>,
        kw_args: HashMap<String, String>, // TODO: Find better name for this
    ) -> Result<Request, anyhow::Error> {
        let client = Client::new();

        let mut url = Url::parse(&format!(
            "{}{}",
            if self.use_tls { "https://" } else { "http://" },
            self.host
        ))?;

        for arg in args {
            url = url.join(arg)?;
        }

        Ok(client.request(method.unwrap_or(Method::POST), url)
            .header("Auth", auth.expect("Expected an authorization key!"))
            .build()?)
    }
}
