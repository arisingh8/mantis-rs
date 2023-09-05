use crate::sample::FileBasedSampleReader;
use crate::sample::StringBasedSampleReader;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct IndexInfo {}

#[derive(Debug)]
pub enum IndexError {
    MaxSizeReached,
    MemoryError,
    DiskError,
    FileError,
    SnapshotError,
}

pub trait Index {
    fn insert(&self, sample: impl FileBasedSampleReader) -> Result<(), IndexError>;
    fn query(&self, sample: impl StringBasedSampleReader) -> Result<Vec<String>, IndexError>;
    fn get_info(&self) -> IndexInfo;
    fn save(&self) -> Result<(), IndexError>;
    fn load(
        index_directory_path: PathBuf,
        internal_insert_threads: usize,
        internal_query_threads: usize,
    ) -> Result<Self, IndexError>
    where
        Self: Sized;
    fn new(
        index_directory_path: PathBuf,
        internal_insert_threads: usize,
        internal_query_threads: usize,
        kmer_size: u32,
    ) -> Result<Self, IndexError>
    where
        Self: Sized;
    fn sync(&mut self) -> Result<bool, IndexError>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexConfig {
    pub uuid: String,
    pub kmer_size: u32,
}

impl IndexConfig {
    pub fn new(uuid: String, kmer_size: u32) -> Self {
        Self { uuid, kmer_size }
    }
}