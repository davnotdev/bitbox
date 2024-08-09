use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct FileInfo {
    relpath: PathBuf,
    size: usize,
}

fn get_dir_files(path: &Path) -> Result<Vec<FileInfo>> {
    let mut res = vec![];

    for dir in fs::read_dir(path)? {
        let dir = dir?;
        let file_type = dir.file_type().unwrap();
        if file_type.is_file() {
            let relpath = dir.path().to_owned();
            let relpath = relpath
                .components()
                .take(relpath.components().count() - path.components().count())
                .collect();
            let size = dir.metadata()?.len() as usize;
            res.push(FileInfo { relpath, size })
        } else if file_type.is_dir() {
            let mut files = get_dir_files(&dir.path())?;
            res.append(&mut files);
        } else if file_type.is_symlink() {
            unimplemented!("push symlinks")
        } else {
            unreachable!()
        }
    }

    Ok(res)
}

pub struct FileChunk {
    pub index: usize,
    pub hash: u64,
    pub bytes: Vec<u8>,
}

pub struct Chunker {
    path: PathBuf,
    files: Vec<FileInfo>,
    worker: Worker<FileChunk>,
}

impl Chunker {
    pub fn new(path: &Path, files: Vec<FileInfo>, worker: Worker<FileChunk>) -> Self {
        Self {
            path: path.to_owned(),
            files,
            worker,
        }
    }

    pub fn run(&self) -> Result<()> {
        let mut buf = [0u8; MAX_CHUNK_SIZE];
        for file_info in self.files.iter() {
            let mut path = self.path.clone();
            path.push(&file_info.relpath);

            let mut hasher = DefaultHasher::new();
            path.hash(&mut hasher);
            let hash = hasher.finish();

            let mut file = File::open(path)?;
            let mut index = 0;
            loop {
                let bytes_read = file.read(&mut buf)?;
                if bytes_read == 0 {
                    break;
                }
                let bytes = buf[0..bytes_read].to_vec();
                let chunk = FileChunk { index, hash, bytes };
                index += 1;
                self.worker.push(chunk);
            }
        }

        Ok(())
    }
}
