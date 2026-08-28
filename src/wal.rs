use crc32fast::Hasher;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, ErrorKind, Read, Write};

pub struct Wal {
    writer: BufWriter<File>,
}

impl Wal {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Wal {
            writer: BufWriter::new(file),
        })
    }

    pub fn append_record(&mut self, record: &[u8]) -> io::Result<()> {
        let length = record.len() as u32;
        let length_bytes = length.to_be_bytes();

        // calculate checksum
        let mut hasher = Hasher::new();
        hasher.update(record);
        let checksum = hasher.finalize();
        let checksum_bytes = checksum.to_be_bytes();

        self.writer.write_all(&length_bytes)?;
        self.writer.write_all(&checksum_bytes)?;
        self.writer.write_all(record)?;

        self.writer.flush()?;
        self.writer.get_mut().sync_data()?;

        Ok(())
    }

    pub fn recover(path: &str) -> io::Result<Vec<Vec<u8>>> {
        let mut file = File::open(path)?;
        let mut recovered_records = Vec::new();

        loop {
            let mut length_bytes = [0u8; 4];

            match file.read_exact(&mut length_bytes) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }

            let length = u32::from_be_bytes(length_bytes) as usize;

            let mut checksum_bytes = [0u8; 4];
            file.read_exact(&mut checksum_bytes)?;
            let disk_checksum = u32::from_be_bytes(checksum_bytes);

            let mut record = vec![0u8; length];
            file.read_exact(&mut record)?;

            let mut hasher = Hasher::new();
            hasher.update(&record);
            let calculated_checksum = hasher.finalize();

            if calculated_checksum != disk_checksum {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "WAL Corruption Detected: Checksum mismatch",
                ));
            }

            recovered_records.push(record);
        }

        Ok(recovered_records)
    }
}
