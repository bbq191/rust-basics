/// 记录数据的位置索引
#[derive(Clone, Copy, Debug)]
pub struct LogDataPos {
    pub(crate) file_id: u64,
    pub(crate) offset: u64,
}
