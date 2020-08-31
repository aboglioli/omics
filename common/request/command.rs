use serde::Serialize;

#[derive(Serialize)]
pub struct CommandResponse {
    ok: bool,
}

impl Default for CommandResponse {
    fn default() -> Self {
        CommandResponse { ok: true }
    }
}
