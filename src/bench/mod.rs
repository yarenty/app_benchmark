use serde_derive::{Deserialize, Serialize};

pub mod analysis;
pub mod collector;
pub mod graphs;

/// CSV output record
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    time: &'a str,
    cpu: &'a str,
    mem: &'a str,
}
