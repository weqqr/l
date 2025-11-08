use std::error::Error;

use glam::ivec3;

use crate::map::{Map, PostgresBackend};

pub mod map;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(dsn) = std::env::args().nth(1) else {
        eprintln!("dsn required");
        std::process::exit(1);
    };

    let postgres = PostgresBackend::new(dsn)?;
    let map = Map::new(postgres);

    let block = map.get_block(ivec3(0, 0, 0))?;

    Ok(())
}
