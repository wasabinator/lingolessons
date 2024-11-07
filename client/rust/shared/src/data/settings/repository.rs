use log::warn;
use crate::domain::settings::SettingRepository;
use crate::{ArcMutex, Run};
use crate::data::db::Db;
use crate::data::settings::db::{Setting, SettingDao};

impl SettingRepository {
    #[allow(dead_code)]
    pub(in crate::data) fn new(db: ArcMutex<Db>) -> Self {
        SettingRepository {
            db: db.clone(),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn get_string(&self, key: &str) -> Option<String> {
        match self.db.clone().run(|db| db.get(key)).await {
            Ok(Setting::Text(text)) => {
                Some(text)
            },
            Err(err) => {
                warn!("Error getting setting {}: {}", key, err);
                None
            },
            _ => {
                None
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn put_string(&self, key: &str, value: String) {
        let _ = self.db.run(
            |db| db.put(key, Setting::Text(value))
        ).await;
    }

    #[allow(dead_code)]
    pub(crate) async fn get_timestamp(&self, key: &str) -> Option<u64> {
        match self.db.clone().run(|db| db.get(key)).await {
            Ok(Setting::Number(number)) => {
                Some(number)
            },
            Err(err) => {
                warn!("Error getting setting {}: {}", key, err);
                None
            },
            _ => {
                None
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn put_timestamp(&self, key: &str, value: u64) {
        let _ = self.db.run(
            |db| db.put(key, Setting::Number(value))
        ).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::{arc_mutex, common::time::UnixTimestamp};

    use super::*;

    #[tokio::test]
    async fn test_repository() {
        let repo = &SettingRepository::new(
            arc_mutex(Db::open("test".into()).unwrap()),
        );

        let key1 = "key1".to_string();
        let key2 = "key2".to_string();

        repo.put_string(&key1, "abc".into()).await;

        // get existing value
        let setting = repo.get_string(&key1).await;
        assert_eq!("abc".to_string(), setting.unwrap());

        // get existing value as wrong type
        let setting = repo.get_timestamp(&key1).await;
        assert_eq!(None, setting);

        // try to get non existent value
        let setting = repo.get_string(&key2).await;
        assert_eq!(None, setting);

        // replace with a timestamp
        let timestamp = UnixTimestamp::now();
        repo.put_timestamp(&key1, timestamp).await;

        // Now fetch it
        let setting = repo.get_timestamp(&key1).await;
        assert_eq!(timestamp, setting.unwrap());

        // Try to fetch as the old type
        let setting = repo.get_string(&key1).await;
        assert_eq!(None, setting);
    }
}
