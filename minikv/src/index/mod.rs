use crate::data::log_data_pos::LogDataPos;
/// 抽象数据接口，以后如果添加新的数据结构，只需要实现这个接口即可
pub trait Indexer {
    /// 向索引中存储 key 对应的位置信息     
    fn put(&self, key: Vec<u8>, pos: LogDataPos) -> bool;
    /// 从索引中获取 key 对应的位置信息
    fn get(&self, key: Vec<u8>) -> Option<LogDataPos>;
    /// 从索引中删除 key 对应的位置信息
    fn del(&self, key: Vec<u8>) -> bool;
}
