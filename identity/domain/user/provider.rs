use common::error::Error;

#[derive(Debug, Clone)]
pub enum Provider {
    Local,
    Google,
    Facebook,
}
