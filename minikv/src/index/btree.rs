use super::Indexer;
use crate::data::log_data_pos::LogDataPos;
use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};

/// 封装系统的BTreeMap
pub struct BTree {
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogDataPos>>>,
}
impl BTree {
    /// 初始化 BTree
    pub fn new() -> Self {
        BTree {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, pos: LogDataPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogDataPos> {
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()
    }

    fn del(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let remove_res = write_guard.remove(&key);
        remove_res.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_put() {
        let btree = BTree::new();
        let res = btree.put(
            "".as_bytes().to_vec(),
            LogDataPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(res, true);
    }

    #[test]
    fn test_btree_get() {
        let btree = BTree::new();
        btree.put(
            "".as_bytes().to_vec(),
            LogDataPos {
                file_id: 1,
                offset: 10,
            },
        );
        let res = btree.get("".as_bytes().to_vec());
        println!("{:?}", res);
        assert!(res.is_some());
        assert_eq!(res.unwrap().file_id, 1);
        assert_eq!(res.unwrap().offset, 10);
    }

    #[test]
    fn test_btree_del() {
        let btree = BTree::new();
        btree.put(
            "".as_bytes().to_vec(),
            LogDataPos {
                file_id: 1,
                offset: 10,
            },
        );
        let res = btree.del("".as_bytes().to_vec());
        assert!(res);
        let res = btree.get("".as_bytes().to_vec());
        assert!(res.is_none());
        let res = btree.del("not exist".as_bytes().to_vec());
        assert!(!res);
    }
}
