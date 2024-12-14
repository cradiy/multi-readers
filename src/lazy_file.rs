use std::{
    fs::File,
    io::{Read, Seek},
    path::{Path, PathBuf},
};

pub enum LazyFile {
    Path(PathBuf),
    File(File),
}
impl LazyFile {
    pub fn new(path: impl AsRef<Path>) -> Self {
        LazyFile::Path(path.as_ref().to_path_buf())
    }
    pub fn get(&mut self) -> std::io::Result<&mut File> {
        match self {
            LazyFile::Path(path) => {
                let file = File::open(path)?;
                *self = LazyFile::File(file);
                self.get()
            }
            LazyFile::File(file) => Ok(file),
        }
    }
}

impl Read for LazyFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.get()?.read(buf)
    }
}
impl Seek for LazyFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.get()?.seek(pos)
    }
}
