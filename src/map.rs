use std::sync::Mutex;

use glam::IVec3;
use postgres::{Client, NoTls};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("block not found")]
    BlockNotFound,

    #[error("postgres error: {0}")]
    Postgres(#[from] postgres::Error),
}

pub struct Map {
    backend: Mutex<Box<dyn MapBackend>>,
}

impl Map {
    pub fn new(backend: impl MapBackend) -> Self {
        Self {
            backend: Mutex::new(Box::new(backend)),
        }
    }

    pub fn get_block(&self, pos: IVec3) -> Result<Block, Error> {
        let data = self.backend.lock().unwrap().get_block_data(pos)?;
        Block::parse_data(&data)
    }
}

pub trait MapBackend: 'static {
    fn get_block_data(&mut self, pos: IVec3) -> Result<Vec<u8>, Error>;
}

pub struct Block {
    data: Vec<u8>,
}

impl Block {
    pub fn parse_data(data: &[u8]) -> Result<Self, Error> {
        unimplemented!()
    }
}

pub struct PostgresBackend {
    client: Client,
}

impl PostgresBackend {
    pub fn new(dsn: String) -> Result<Self, Error> {
        let client = postgres::Client::connect(&dsn, NoTls)?;

        Ok(Self { client })
    }
}

impl MapBackend for PostgresBackend {
    fn get_block_data(&mut self, pos: IVec3) -> Result<Vec<u8>, Error> {
        const SQL: &str = "
            SELECT data
            FROM blocks
            WHERE posx = ?
              AND posy = ?
              AND posz = ?
            LIMIT 1";

        let row = self.client.query_one(SQL, &[&pos.x, &pos.y, &pos.z])?;
        let data = row.get(0);

        Ok(data)
    }
}
