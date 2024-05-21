use self::error::RepositoryError;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{collections::HashMap, io::BufReader, path::Path};
pub mod error;

pub struct Repository<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash + Clone,
    V: Serialize + DeserializeOwned + Clone,
{
    inner: HashMap<K, V>,
    storage: PathBuf,
}

impl<K, V> Repository<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash + Clone,
    V: Serialize + DeserializeOwned + Clone + Eq,
{
    #[tracing::instrument]
    pub fn from_path(path: impl AsRef<Path> + Debug) -> Result<Self, RepositoryError> {
        let reader = BufReader::new(OpenOptions::new().read(true).open(&path)?);
        let map: HashMap<K, V> = serde_json::from_reader(reader)?;
        Ok(Self {
            inner: map,
            storage: path.as_ref().to_path_buf(),
        })
    }

    pub async fn get(&self, key: K) -> Option<V> {
        self.inner.get(&key).cloned() // For primitive types it dereference
    }

    pub async fn insert(&mut self, key: K, value: V) -> Option<V> {
        let insert_result = self.inner.insert(key, value);

        if let Err(cause) = self.flush() {
            tracing::warn!(%cause, "Failed to write repository to storage!");
        }
        insert_result
    }

    fn flush(&self) -> Result<(), RepositoryError> {
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.storage.as_path())?,
        );
        serde_json::to_writer(&mut writer, &self.inner)?;
        writer.flush()?;
        Ok(())
    }

    pub async fn get_by_value(&self, value: V) -> Option<(K, V)> {
        self.inner
            .iter()
            .find(|(_, v)| (*v).eq(&value))
            .map(|(k, v)| (k.clone(), v.clone()))
    }

    pub async fn remove(&mut self, key: K) {
        self.inner.remove(&key);
        if let Err(cause) = self.flush() {
            tracing::warn!(%cause, "Failed to write repository to storage!");
        }
    }
}
