use crate::index::{Index, IndexError, IndexInfo};
use crate::sample::{FileBasedSampleReader, StringBasedSampleReader};
use ahash::RandomState as aRandomState;
use cqfrs::{BuildReversableHasher, CountingQuotientFilter};
use parking_lot::{Condvar, Mutex};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufWriter;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

const MAX_SAMPLES: usize = 6400;
const NUM_METACOLORS: usize = MAX_SAMPLES / 64;

pub struct Mantis<'a> {
    on_disk: Mutex<CountingQuotientFilter<'a, BuildReversableHasher>>,
    buffer: Mutex<CountingQuotientFilter<'a, BuildReversableHasher>>,
    index_directory_path: PathBuf,
    sample_ids: Mutex<Vec<String>>,
    color_matrix: Mutex<Vec<u64>>,
    color_ids: Mutex<HashMap<u64, usize, aRandomState>>,
    metacolor_table: Mutex<Vec<Metacolor>>,
    metacolor_ids: Mutex<HashMap<u64, usize, aRandomState>>,
    delete_ids: Mutex<Vec<usize>>,
    epoch_counter: AtomicU64,
    dirty: AtomicBool,
    condition: Condvar,
    flush_condition: (Mutex<bool>, Condvar),
}

#[derive(Clone)]
struct Metacolor {
    ref_count: u8,
    metacolor: [MetacolorChunk; NUM_METACOLORS],
}

#[derive(Clone, Copy, Hash)]
#[repr(packed)]
struct MetacolorChunk {
    //partition: u8,
    color_id: u32,
}

impl Hash for Metacolor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.metacolor.hash(state);
    }
}

impl Index for Mantis<'_> {
    fn new(
        index_directory_path: PathBuf,
        _internal_insert_threads: usize,
        _internal_query_threads: usize,
        kmer_size: u32,
    ) -> Result<Self, IndexError> {
        Ok(Self {
            buffer: Mutex::new(
                CountingQuotientFilter::new(
                    31,
                    31,
                    kmer_size as u64 * 2,
                    true,
                    BuildReversableHasher::default(),
                )
                .expect("couldn't create in-memory buffer!"),
            ),
            on_disk: Mutex::new(
                CountingQuotientFilter::new_file(
                    10,
                    10,
                    kmer_size as u64 * 2,
                    true,
                    BuildReversableHasher::default(),
                    index_directory_path.join("dbg.cqf"),
                )
                .expect("Failed to make on disk"),
            ),
            index_directory_path: index_directory_path.clone(),
            sample_ids: Mutex::new(Vec::new()),
            dirty: AtomicBool::new(false),
            color_matrix: Mutex::new(Vec::new()),
            color_ids: Mutex::new(HashMap::with_hasher(Default::default())),
            metacolor_table: Mutex::new(Vec::new()),
            metacolor_ids: Mutex::new(HashMap::with_hasher(Default::default())),
            delete_ids: Mutex::new(Vec::new()),
            epoch_counter: AtomicU64::new(0),
            condition: Condvar::new(),
            flush_condition: (Mutex::new(false), Condvar::new()),
        })
    }

    fn insert(&self, mut sample: impl FileBasedSampleReader) -> Result<(), IndexError> {
        let id;
        {
            let mut ids = self.sample_ids.lock();
            id = ids.len() as u64;
            ids.push(sample.get_sample_id());
            while id / 64 > self.epoch_counter.load(std::sync::atomic::Ordering::SeqCst) {
                self.condition.wait(&mut ids);
            }
        }
        sample.open().expect("failed to open sample!");
        /* Insert Sample */
        {
            let id = id % 64;
            let buffer = &mut self.buffer.lock();
            for kmer in sample {
                let color = buffer.query(kmer).count;
                if color != 0 {
                    match buffer.set_count(kmer, color | 1 << id) {
                        Err(_) => {
                            buffer.resize().expect("failed to resize");
                        }
                        _ => {}
                    }
                } else {
                    match buffer.insert(kmer, color | 1 << id) {
                        Err(_) => {
                            buffer.resize().expect("failed to resize");
                        }
                        _ => {}
                    }
                }
            }
        }
        if id % 64 != 63 {
            return Ok(());
        }
        let mut buffer = self.buffer.lock();
        println!("flushing");
        let mut disk_cqf = self.on_disk.lock();
        let new_cqf = CountingQuotientFilter::merge_file_cb(
            self,
            &*buffer,
            &*disk_cqf,
            self.index_directory_path.join("dbg.cqf"),
            Self::merge_cb,
        )
        .expect("failed to merge!");

        // tracing::info!("finished merging");
        buffer.clear();
        *disk_cqf = new_cqf;
        self.dirty.store(true, Ordering::SeqCst);
        let partition_id = self.epoch_counter.fetch_add(1, Ordering::SeqCst);
        let mut buffer_file = BufWriter::new(
            File::create(
                self.index_directory_path
                    .join(format!("color-{}.table", partition_id)),
            )
            .expect("failed to create color buffer file"),
        );
        let mut color_table = self.color_matrix.lock();
        let mut color_ids = self.color_ids.lock();
        bincode::encode_into_std_write(
            color_table.deref(),
            &mut buffer_file,
            bincode::config::standard(),
        )
        .expect("failed to serialize");
        color_table.clear();
        color_ids.clear();

        self.condition.notify_all();

        Ok(())
    }

    fn query(&self, sample: impl StringBasedSampleReader) -> Result<Vec<String>, IndexError> {
        todo!();
    }

    fn get_info(&self) -> IndexInfo {
        todo!()
    }

    fn save(&self) -> Result<(), IndexError> {
        Ok(())
    }

    fn load(
        _index_directory_path: PathBuf,
        _internal_insert_threads: usize,
        _internal_query_threads: usize,
    ) -> Result<Self, IndexError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn sync(&mut self) -> Result<bool, IndexError> {
        let (lock, _cvar) = &self.flush_condition;
        // tracing::info!("syncing");
        let mut flushed = lock.lock();
        // tracing::info!("sync: flush: {}", *flushed);
        if *flushed {
            *flushed = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Mantis<'_> {
    /// This function gets called before every insert into the merged CQF
    /// a is the in memory CQF
    /// b is the on disk CQF
    pub fn merge_cb(
        &self,
        _new_cqf: &mut CountingQuotientFilter<'_, BuildReversableHasher>,
        _a: &CountingQuotientFilter<'_, BuildReversableHasher>,
        _b: &CountingQuotientFilter<'_, BuildReversableHasher>,
        a_quotient: u64,
        a_remainder: u64,
        a_count: &mut u64,
        b_quotient: u64,
        b_remainder: u64,
        b_count: &mut u64,
    ) {
        // We're not inserting from buffer on this run
        if a_quotient > b_quotient || (a_quotient == b_quotient && a_remainder > b_remainder) {
            return;
        }

        // color matrix is a vector of color classes, color ids let's us check if we've seen a color before
        let mut color_matrix = self.color_matrix.lock();
        let mut color_ids = self.color_ids.lock();

        let mut metacolor_table = self.metacolor_table.lock();
        let mut metacolor_ids = self.metacolor_ids.lock();

        let mut delete_ids = self.delete_ids.lock();

        // handle buffer ops
        let partition_id = self.epoch_counter.load(Ordering::SeqCst);
        let color_id = match color_ids.entry(*a_count) {
            Entry::Occupied(occupied) => occupied.get().clone(),
            Entry::Vacant(vacant) => {
                color_matrix.push(*a_count);
                vacant.insert(color_matrix.len()).clone()
            }
        };
        color_matrix.push(*a_count);

        // kmer is in both CQFs, we either split or join
        if a_quotient == b_quotient && a_remainder == b_remainder {
            let current_color_index = *b_count;
            let current_color = metacolor_table
                .get_mut(current_color_index as usize)
                .unwrap();

            if current_color.ref_count == 1 {
                // since we are changing the color, let's get rid of the old hash
                let current_hash = metacolor_ids.hasher().hash_one(&current_color);
                metacolor_ids.remove(&current_hash);

                // let's change the color and check on the new hash
                current_color.metacolor[partition_id as usize].color_id = color_id as u32;
                let new_hash = metacolor_ids.hasher().hash_one(current_color);

                // check if we actually made the color earlier
                match metacolor_ids.entry(new_hash) {
                    Entry::Occupied(occupied) => {
                        // we did make one!
                        *b_count = *occupied.get() as u64;
                        delete_ids.push(current_color_index as usize);
                    }
                    Entry::Vacant(vacant) => {
                        // we didn't make one.
                        vacant.insert(current_color_index as usize);
                    }
                };
                *a_count = 0;
                return;
            } else {
                // drop the old ref count
                current_color.ref_count -= 1;

                // make new color
                let mut new_color = current_color.clone();
                new_color.metacolor[partition_id as usize].color_id = color_id as u32;

                // new numbers for new color
                let new_hash = metacolor_ids.hasher().hash_one(&new_color);
                let new_id = match delete_ids.pop() {
                    Some(deleted) => {
                        metacolor_table[deleted] = new_color;
                        deleted
                    }
                    None => {
                        metacolor_table.push(new_color);
                        metacolor_table.len() - 1
                    }
                };

                // update hashes
                metacolor_ids.insert(new_hash, new_id);

                // change colors for cqf insert
                *a_count = 0;
                *b_count = new_id as u64;
                return;
            }
        }
        // kmer exists in memory but not on disk
        let mut new_color = Metacolor {
            ref_count: 1,
            metacolor: [MetacolorChunk { color_id: 0 }; NUM_METACOLORS],
        };
        new_color.metacolor[partition_id as usize].color_id = color_id as u32;
        // new numbers for new color
        let new_hash = metacolor_ids.hasher().hash_one(&new_color);

        // check if we actually made the color earlier, update hashes if necessary
        let new_id = match metacolor_ids.entry(new_hash) {
            Entry::Occupied(occupied) => {
                // we did make one!
                *occupied.get() as u64
            }
            Entry::Vacant(vacant) => {
                // we didn't make one.
                *vacant.insert(match delete_ids.pop() {
                    Some(deleted) => {
                        metacolor_table[deleted] = new_color;
                        deleted
                    }
                    None => {
                        metacolor_table.push(new_color);
                        metacolor_table.len() - 1
                    }
                }) as u64
            }
        };
        *a_count = new_id;
        return;
    }
}
