use super::*;
use zstd::bulk::Compressor;

pub struct ZippedChunk {
    pub index: usize,
    pub hash: u64,
    pub bytes: Vec<u8>,
}

pub struct Zipper {
    chunk_stealer: Stealer<FileChunk>,
    net_channel: NetChannel,
}

impl Zipper {
    pub fn new(chunk_stealer: Stealer<FileChunk>, net_channel: NetChannel) -> Self {
        Self {
            chunk_stealer,
            net_channel,
        }
    }

    pub fn run(&self) {
        loop {
            while let Steal::Success(chunk) = self.chunk_stealer.steal() {
                let mut compressor = Compressor::new(7).unwrap();
                let zipped = compressor.compress(&chunk.bytes).unwrap();
                self.net_channel
                    .send(ZippedChunk {
                        index: chunk.index,
                        hash: chunk.hash,
                        bytes: zipped,
                    })
                    .unwrap();
            }
        }
    }
}
