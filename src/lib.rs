use std::collections::HashMap;

use reqwest::{Client, Method, Request};
use url_builder::URLBuilder;

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

pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn add_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
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
        query_params: QueryParams,
    ) -> Result<Request, anyhow::Error> {
        let client = Client::new();

        let scheme = if self.use_tls { "https://" } else { "http://" };

        let mut ub = URLBuilder::new();

        ub.set_protocol(scheme);
        ub.set_host(&self.host);

        for arg in args {
            ub.add_route(arg);
        }
        
        for (key, value) in query_params.params {
            ub.add_param(&key, &value);
        }
        
        let url = ub.build();

        Ok(client.request(method.unwrap_or(Method::POST), url)
            .header("Auth", auth.expect("Expected an authorization key!"))
            .build()?)
    }
}
