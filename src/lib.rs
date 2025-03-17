use std::collections::HashMap;

use anyhow::{anyhow, Context};
use log::info;
use reqwest::{Client, Method, Response};
use url_builder::URLBuilder;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("%23{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}

#[derive(Debug)]
pub struct Pane {
    pub name: String,
    pub bg_color: Color,
    pub conn: Connection,
    token: Option<String>,
}

impl Pane {
    pub async fn init(name: &str, bg_color: Color) -> Result<Pane, anyhow::Error> {
        let conn = Connection::new("ttds.tali.network", true);

        let args = ["pane", name, "create"];
        let binding = args.map(|e| e.into());

        let response = conn.request(
            &binding,
            None,
            None,
            QueryParams::new().add_param("color", bg_color.to_string()),
        );

        match response.await {
            Ok(resp) => Ok(Pane {
                name: name.into(),
                bg_color,
                conn,
                token: Some(
                    resp.text()
                        .await
                        .context("Could not decode response token as string")?,
                ),
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn rect(
        &self,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        color: Color,
    ) -> Result<Response, anyhow::Error> {
        self.conn
            .request(
                &["rect".into()],
                None,
                Some(self.get_token()?.as_str()),
                QueryParams::new()
                    .add_param("x", x.to_string())
                    .add_param("y", y.to_string())
                    .add_param("w", w.to_string())
                    .add_param("h", h.to_string())
                    .add_param("color", color.to_string()),
            )
            .await
    }

    pub fn get_token(&self) -> Result<String, anyhow::Error> {
        match &self.token {
            Some(token) => Ok(token.clone()),
            None => Err(anyhow!(
                "Token not yet initialized! Did you succesfully open a connection?"
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

#[derive(Debug)]
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
        &self,
        args: &[String], // TODO: Find better name for this
        method: Option<Method>,
        auth: Option<&str>,
        query_params: QueryParams,
    ) -> Result<Response, anyhow::Error> {
        let client = Client::new();

        let scheme = if self.use_tls { "https" } else { "http" };

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

        info!("Sending request URL: {url}");

        let request_build = match auth {
            Some(auth) => client
                .request(method.unwrap_or(Method::POST), url)
                .header("Auth", auth)
                .build(),
            None => client.request(method.unwrap_or(Method::POST), url).build(),
        };

        match request_build {
            Ok(request) => Ok(client.execute(request).await?),
            Err(err) => Err(err.into()),
        }
    }
}
