use serde::{Deserialize, Serialize};

pub mod multipart;

#[derive(Serialize, Deserialize, Debug)]
pub struct CtsFile {
    pub path: String,
    pub filename: String,
}