use super::SessionManager;
use crate::prelude::*;

use redis::{Client, Commands};

const YEAR_IN_SECS: u64 = 365 * 60 * 60 * 24;

impl SessionManager for Client {
    fn insert(&self, id: i32, key: String) -> Result<()> {
        let mut cnn = self.get_connection()?;
        let _: () = cnn.set_ex(id, key, YEAR_IN_SECS)?;
        Ok(())
    }

    fn insert_for(&self, id: i32, key: String, time: Duration) -> Result<()> {
        let mut cnn = self.get_connection()?;
        let _: () = cnn.set_ex(id, key, time.as_secs())?;
        Ok(())
    }

    fn remove(&self, id: i32) -> Result<()> {
        let mut cnn = self.get_connection()?;
        let _: () = cnn.del(id)?;
        Ok(())
    }

    fn get(&self, id: i32) -> Option<String> {
        let mut cnn = self.get_connection().ok()?;
        let key = cnn.get(id).ok()?;
        key
    }

    fn clear_all(&self) -> Result<()> {
        let mut cnn = self.get_connection()?;
        redis::Cmd::new().arg("FLUSHDB").exec(&mut cnn)?;
        Ok(())
    }

    fn clear_expired(&self) -> Result<()> {
        Ok(())
    }
}
