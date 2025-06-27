// core/src/formats/dummy.rs
use crate::{Edits, SaveGame};
use anyhow::Result;

#[derive(Debug, Default)]
pub struct DummySave {
    pub coins: u32,
}

impl SaveGame for DummySave {
    fn from_bytes(data: &[u8]) -> Result<Self> {
        // Very fake: first 4 bytes = coins, little-endian
        Ok(Self {
            coins: u32::from_le_bytes(data[0..4].try_into()?),
        })
    }

    fn patch(&mut self, edits: &Edits) {
        if let Some(new) = edits.coins {
            self.coins = new;
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.coins.to_le_bytes().to_vec())
    }
}
