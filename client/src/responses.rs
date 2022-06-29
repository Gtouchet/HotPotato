use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Welcome {
    pub version: u8,
}
