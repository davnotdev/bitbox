use super::*;

mod chunker;

const MAX_CHUNK_SIZE: usize = 128000000;

pub struct PushConfig {
    safe_mode: bool,
}

#[derive(Error, Debug)]
pub enum PushError {}

pub fn cmd_push(cmd_config: &PushConfig) -> Result<()> {
    Ok(())
}
