#[derive(Debug, Clone)]
pub enum Provider {
    Local,
    Google,
    Facebook,
}

impl ToString for Provider {
    fn to_string(&self) -> String {
        match self {
            Provider::Local => "local".to_owned(),
            Provider::Google => "google".to_owned(),
            Provider::Facebook => "facebook".to_owned(),
        }
    }
}
