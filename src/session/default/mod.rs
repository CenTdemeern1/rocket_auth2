use super::AuthKey;
use super::SessionManager;
use crate::prelude::*;
use chashmap::CHashMap;

impl SessionManager for CHashMap<i32, AuthKey> {
    fn insert(&self, id: i32, key: String) -> Result<()> {
        self.insert(id, key.into());
        Ok(())
    }

    fn insert_for(&self, id: i32, key: String, time: Duration) -> Result<()> {
        let key = AuthKey {
            expires: time.as_secs() as i64,
            secret: key,
        };
        self.insert(id, key);
        Ok(())
    }

    fn remove(&self, id: i32) -> Result<()> {
        self.remove(&id);
        Ok(())
    }

    fn get(&self, id: i32) -> Option<String> {
        let key = self.get(&id)?;
        Some(key.secret.clone())
    }

    fn clear_all(&self) -> Result<()> {
        self.clear();
        Ok(())
    }

    fn clear_expired(&self) -> Result<()> {
        let time = now();
        self.retain(|_, auth_key| auth_key.expires > time);
        Ok(())
    }
}
