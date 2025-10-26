use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::{Error, Result};
use chrono::{DateTime, Utc, serde::ts_microseconds};
use serde::Deserialize;
use ta::{Close, High, Low, Open, Volume};

#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    #[serde(alias = "open_price")]
    open: f64,
    #[serde(alias = "high_price")]
    high: f64,
    #[serde(alias = "low_price")]
    low: f64,
    #[serde(alias = "close_price")]
    close: f64,
    #[serde(rename = "quote_asset_volume")]
    volume: f64,
    #[allow(dead_code)]
    #[serde(with = "ts_microseconds")]
    open_time: DateTime<Utc>,
}

impl Open for Data {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for Data {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for Data {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for Data {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for Data {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub(crate) fn get_data_from_file(filepath: PathBuf) -> Result<Vec<Data>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| Error::msg(e.to_string()))
}
