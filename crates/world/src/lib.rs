mod map;
mod meta;
mod sqlite;

use std::path::{Path, PathBuf};

pub use self::map::*;
pub use self::meta::*;
pub use self::sqlite::*;

pub struct World {
    pub name: String,
    pub meta: WorldMeta,
    pub map: Map,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("metadata error: {0}")]
    Metadata(#[from] MetaError),

    #[error("map error: {0}")]
    MapError(#[from] MapError),

    #[error("unknown map backend: {0}")]
    UnknownBackend(String),

    #[error("invalid path: {0}")]
    InvalidPath(PathBuf),
}

impl World {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();

        let name = path
            .components()
            .next_back()
            .ok_or(Error::InvalidPath(path.to_path_buf()))?
            .as_os_str()
            .to_string_lossy()
            .to_string();

        let meta_path = path.join("world.mt");
        let meta = WorldMeta::open(meta_path)?;
        let backend = meta.get_str("backend").unwrap();

        let map = match backend {
            "sqlite3" => {
                let sqlite_path = path.join("map.sqlite");
                let sqlite = SqliteBackend::new(sqlite_path)?;
                Map::new(sqlite)
            }
            _ => {
                return Err(Error::UnknownBackend(backend.to_owned()));
            }
        };

        Ok(Self { name, meta, map })
    }
}
