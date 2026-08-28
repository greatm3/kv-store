use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

pub struct Wal {
    writer: BufWriter<File>,
}

impl Wal {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Wal {
            writer: BufWriter::new(file),
        })
    }
}
