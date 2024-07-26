pub mod file_io;
use crate::errors::Result;

pub trait IOManager: Send + Sync {
    /// 写入字节数据到文件中
    fn write(&self, buf: &[u8]) -> Result<usize>;

    /// 从文件的指定位置读取数据到 buf 中
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;

    /// 持久化数据到磁盘
    fn sync(&self) -> Result<()>;
}
