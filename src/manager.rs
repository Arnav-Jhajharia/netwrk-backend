use tokio::sync::RwLock;
use std::time::SystemTime;
use lru::LruCache;


pub const MAX_CACHE_SIZE: usize = 3;
pub struct StateManager{
    pub(crate) lru_cache : RwLock<LruCache<String , SystemTime>>,
}


impl StateManager {
    pub async fn cache_upsert(&self, key: String, value : SystemTime) {

        let mut cache = self.lru_cache.write().await ;
        cache.put(key , value);
    }



    pub async fn cache_get(&self, this:String) -> bool{
        self.lru_cache.write().await.get(&this).is_some()
    }
}