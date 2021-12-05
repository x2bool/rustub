use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use crate::common::{PAGE_SIZE, PageId};

pub struct DiskManager {
    log_file_name: String,
    db_file_name: String,
    log_file: File,
    db_file: File,
}

pub enum DiskManagerError {
    InvalidFile,
    Io(std::io::Error)
}

impl From<std::io::Error> for DiskManagerError {
    fn from(err: std::io::Error) -> Self {
        DiskManagerError::Io(err)
    }
}

impl DiskManager {

    /// Creates a new disk manager that writes to the specified database file.
    fn new(db_file_name: impl ToString) -> Result<Self, DiskManagerError> {
        let db_file_name = db_file_name.to_string();
        let position = db_file_name.rfind(".");

        let n = match position {
            Some(n) => n,
            None => return Err(DiskManagerError::InvalidFile)
        };

        let log_file_name = db_file_name[0..n].to_string() + ".log";

        // try open existing log file
        let log_io = OpenOptions::new()
            .append(true)
            .read(true)
            .open(&log_file_name);

        let log_file = match log_io {
            Ok(file) => file,
            Err(_) => {
                // try create new log file
                let log_io = OpenOptions::new()
                    .create_new(true)
                    .append(true)
                    .read(true)
                    .open(&log_file_name);

                match log_io {
                    Ok(file) => file,
                    Err(e) => return Err(DiskManagerError::Io(e))
                }
            }
        };

        // try open existing log file
        let db_io = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&db_file_name);

        let db_file = match db_io {
            Ok(file) => file,
            Err(_) => {
                // try create new log file
                let db_io = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .read(true)
                    .open(&log_file_name);

                match db_io {
                    Ok(file) => file,
                    Err(e) => return Err(DiskManagerError::Io(e))
                }
            }
        };

        Ok(DiskManager{
            log_file_name,
            db_file_name,
            log_file,
            db_file
        })
    }

    /// Write a page to the database file.
    fn write_page(&mut self, page_id: PageId, page_data: &[u8]) -> Result<(), DiskManagerError> {
        let PageId(id) = page_id;
        let offset = id * PAGE_SIZE;

        self.db_file.seek(SeekFrom::Start(offset as u64))?;
        self.db_file.write_all(page_data)?;

        Ok(())
    }

    /// Read a page from the database file.
    fn read_page(&mut self, page_id: PageId, page_data: &mut [u8]) -> Result<(), DiskManagerError> {
        let PageId(id) = page_id;
        let offset = id * PAGE_SIZE;

        self.db_file.seek(SeekFrom::Start(offset as u64))?;
        self.db_file.read_exact(page_data)?;

        Ok(())
    }

    /// Flush the entire log buffer into disk.
    fn write_log(&mut self, log_data: &[u8]) -> Result<(), DiskManagerError> {
        self.log_file.seek(SeekFrom::End(0))?;
        self.log_file.write_all(log_data)?;

        Ok(())
    }

    /// Read a log entry from the log file.
    fn read_log(&mut self, log_data: &mut [u8], offset: usize) -> Result<(), DiskManagerError> {

        self.log_file.seek(SeekFrom::Start(offset as u64))?;
        self.log_file.read_exact(log_data)?;

        Ok(())
    }

}