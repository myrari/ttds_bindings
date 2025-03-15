use std::collections::HashMap;

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

    pub fn request(
        self,
        args: &[String], // TODO: Find better name for this
        method: Option<&str>,
        auth: Option<&str>,
        kw_args: HashMap<String, String>, // TODO: Find better name for this
    ) -> String {
        todo!()
    }
}
