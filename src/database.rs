use std::collections::HashMap;
use std::{path::PathBuf, sync::Arc};

use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct DeployedMerchandise {
    db: Arc<RwLock<HashMap<String, PathBuf>>>,
}

impl DeployedMerchandise {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// データを非同期で追加
    pub async fn insert(&self, key: String, value: PathBuf) {
        let mut db = self.db.write().await;
        db.insert(key, value);
    }

    /// データを非同期で取得
    pub async fn get(&self, key: &String) -> Option<PathBuf> {
        let db = self.db.read().await;
        db.get(key).cloned()
    }
}

impl Default for DeployedMerchandise {
    fn default() -> Self {
        Self::new()
    }
}
