use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WalOp {
    Put { key: String, value: String },
    Delete { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalRecord {
    pub seq: u64,     // 시퀀스 번호 (단조 증가)
    pub op: WalOp,
}