use super::IOManager;
use crate::errors::Errors;
use crate::errors::Result;
use log::error;
use parking_lot::RwLock;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, os::unix::prelude::FileExt, sync::Arc};

/// 标准系统文件 IO
pub struct FileIO {
    fd: Arc<RwLock<File>>, // 系统文件描述符
}

impl FileIO {
    pub fn new(filename: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(filename)
        {
            Ok(file) => Ok(Self {
                fd: Arc::new(RwLock::new(file)),
            }),
            Err(e) => {
                error!("failed to open file error: {:?}", e);
                Err(Errors::FailedToOpenDataFile)
            }
        }
    }
}
impl IOManager for FileIO {
    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        match write_guard.write(buf) {
            Ok(n) => Ok(n),
            Err(e) => {
                error!("write data to file error: {:?}", e);
                Err(Errors::FailedToWriteDataToFile)
            }
        }
    }

    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(n) => Ok(n),
            Err(e) => {
                error!("read data form file error: {:?}", e);
                Err(Errors::FailedToReadDataFromFile)
            }
        }
    }

    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("sync data to file error: {:?}", e);
            return Err(Errors::FailedToSyncDataFile);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_io_write() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("hello".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let res2 = fs::remove_file(path.clone());
        assert!(res2.is_ok());
    }
    #[test]
    fn test_file_io_read() {
        let path = PathBuf::from("/tmp/b.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("hello".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let mut buf = [0u8; 5];
        let read_res1 = fio.read(&mut buf, 0);
        assert!(read_res1.is_ok());
        assert_eq!(5, read_res1.ok().unwrap());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }
    #[test]
    fn test_file_io_sync() {
        let path = PathBuf::from("/tmp/c.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("hello".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let sync_res = fio.sync();
        assert!(sync_res.is_ok());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }
}
