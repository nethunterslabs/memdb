//! Thread-safe in-memory key-value store. Ideal for development and prototyping.
//! Does not persist to disk.
//!
//! ## Examples
//!
//! ```
//! # #[async_std::main]
//! # async fn main() -> std::io::Result<()> {
//! let mut db = memdb::Memdb::open().await?;
//! db.set("beep", "boop").await?;
//! let val = db.get("beep").await?;
//! assert_eq!(val, Some("boop".as_bytes().to_owned()));
//! # Ok(())
//! # }
//! ```
use dashmap::DashMap;

use std::io;
use std::sync::Arc;

/// Key-value database.
#[derive(Debug, Clone)]
pub struct Memdb {
    hashmap: Arc<DashMap<Vec<u8>, Vec<u8>>>,
}

impl Memdb {
    /// Create a new instance.
    #[inline]
    pub async fn open() -> io::Result<Self> {
        Ok(Self {
            hashmap: Arc::new(DashMap::<Vec<u8>, Vec<u8>>::new()),
        })
    }

    /// Set a value in the database.
    #[inline]
    pub async fn set(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> io::Result<Option<Vec<u8>>> {
        let hashmap = self.hashmap.clone();
        Ok(hashmap.insert(key.as_ref().to_owned(), value.as_ref().to_owned()))
    }

    /// Get a value from the database.
    #[must_use]
    #[inline]
    pub async fn get(&self, key: impl AsRef<[u8]>) -> io::Result<Option<Vec<u8>>> {
        let key = key.as_ref().to_owned();
        let hashmap = &self.hashmap;
        match hashmap.get(&key) {
            Some(value) => {
                let value = value.clone();
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Delete a value from the database.
    #[inline]
    pub async fn del(&mut self, key: impl AsRef<[u8]>) -> io::Result<Option<(Vec<u8>, Vec<u8>)>> {
        let key = key.as_ref().to_owned();
        let hashmap = &mut self.hashmap;
        Ok(hashmap.remove(&key))
    }
}
