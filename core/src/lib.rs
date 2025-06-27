// core/src/lib.rs
use anyhow::Result;

pub mod formats;
/// Generic edits the front-end will pass in.
#[derive(Debug, Default)]
pub struct Edits {
    pub coins: Option<u32>,
    // add more fields later...
}

/// Everything a game-specific module must provide.
pub trait SaveGame {
    fn from_bytes(data: &[u8]) -> Result<Self>
    where
        Self: Sized;
    fn patch(&mut self, edits: &Edits);
    fn to_bytes(&self) -> Result<Vec<u8>>;
}
