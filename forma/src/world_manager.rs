use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use uuid::Uuid;
use world::{Map, SqliteBackend, WorldMeta};

pub struct WorldManager {
    worlds: HashMap<Uuid, Map>,
    path_to_id: HashMap<String, Uuid>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
            path_to_id: HashMap::new(),
        }
    }

    pub fn open(&mut self, path: impl AsRef<Path>) -> Result<Uuid> {
        let meta_path = path.as_ref().join("world.mt");
        let meta = WorldMeta::open(meta_path).context("read world metadata")?;
        let backend = meta.get_str("backend").unwrap();

        let map = match backend {
            "sqlite3" => {
                let sqlite_path = path.as_ref().join("map.sqlite");
                let sqlite = SqliteBackend::new(sqlite_path)?;
                Map::new(sqlite)
            }
            _ => {
                return Err(anyhow!("unknown backend: {backend}"));
            }
        };

        let id = Uuid::new_v4();
        self.worlds.insert(id, map);

        Ok(id)
    }

    pub fn map_by_id(&self, id: Uuid) -> Option<&Map> {
        self.worlds.get(&id)
    }

    pub fn world() {}
}
