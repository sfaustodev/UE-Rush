use sled::{Db, IVec};
use std::path::Path;

pub struct HashCidDb {
    db: Db,
}

impl HashCidDb {
    pub fn new(path: &Path) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn insert(&self, hash: &str, cid: &str) -> Result<(), sled::Error> {
        self.db.insert(hash, cid)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, hash: &str) -> Result<Option<String>, sled::Error> {
        match self.db.get(hash)? {
            Some(ivec) => Ok(Some(String::from_utf8_lossy(&ivec).to_string())),
            None => Ok(None),
        }
    }

    pub fn remove(&self, hash: &str) -> Result<(), sled::Error> {
        self.db.remove(hash)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<(String, String), sled::Error>> {
        self.db.iter().map(|res| {
            res.map(|(k, v)| (
                String::from_utf8_lossy(&k).to_string(),
                String::from_utf8_lossy(&v).to_string(),
            ))
        })
    }
}
