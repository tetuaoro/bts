use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use chrono::{DateTime, Utc, serde::ts_microseconds};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    #[serde(alias = "open_price")]
    pub open: f64,
    #[serde(alias = "high_price")]
    pub high: f64,
    #[serde(alias = "low_price")]
    pub low: f64,
    #[serde(alias = "close_price")]
    pub close: f64,
    #[serde(rename = "quote_asset_volume")]
    pub volume: f64,
    #[allow(dead_code)]
    #[serde(with = "ts_microseconds")]
    pub open_time: DateTime<Utc>,
}

pub(crate) fn get_data_from_file(filepath: PathBuf) -> Result<Vec<Data>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| anyhow::Error::msg(e.to_string()))
}
