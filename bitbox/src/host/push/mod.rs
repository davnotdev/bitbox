use super::*;
use crossbeam::deque::{Steal, Stealer, Worker};

mod chunker;
mod net;
mod zipper;

pub use chunker::{Chunker, FileChunk};
pub use net::NetChannel;
pub use zipper::{ZippedChunk, Zipper};

const MAX_CHUNK_SIZE: usize = 128000000;

pub struct PushConfig {
    safe_mode: bool,
}

#[derive(Error, Debug)]
pub enum PushError {}

pub fn cmd_push(cmd_config: &PushConfig) -> Result<()> {
    Ok(())
}
