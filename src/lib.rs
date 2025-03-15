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
        return match &self.token {
            Some(token) => Ok(token.clone()),
            None => Err(String::from(
                "Token not yet initialized! Did you succesfully open a connection?",
            )),
        };
    }
}
