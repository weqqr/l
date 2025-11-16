use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use uuid::Uuid;
use world::World;

pub struct WorldManager {
    worlds: HashMap<Uuid, World>,
    path_to_id: HashMap<PathBuf, Uuid>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
            path_to_id: HashMap::new(),
        }
    }

    pub fn open(&mut self, path: impl AsRef<Path>) -> Result<Uuid> {
        let path = path.as_ref().canonicalize()?.to_path_buf();

        if let Some(id) = self.path_to_id.get(&path) {
            return Ok(*id);
        }

        let world = World::open(&path).context("Unable to open world")?;

        let id = Uuid::new_v4();
        self.worlds.insert(id, world);

        Ok(id)
    }

    pub fn world_by_id(&self, id: Uuid) -> Option<&World> {
        self.worlds.get(&id)
    }
}
