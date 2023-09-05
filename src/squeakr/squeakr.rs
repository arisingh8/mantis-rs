#[repr(C)]
pub struct FileInfo {
    pub fd: ::std::os::raw::c_int,
    pub filepath: *mut ::std::os::raw::c_char,
}
#[repr(C)]
pub struct LocalCounter {
    pub counter: i64,
    pub padding: [i64; 7usize],
}
pub type Lctr = LocalCounter;

#[repr(C)]
pub struct PartitionedCounter {
    pub local_counters: *mut Lctr,
    pub global_counter: *mut i64,
    pub num_counters: u32,
    pub threshold: i32,
}

#[repr(C)]
pub struct WaitTimeData {
    pub total_time_single: u64,
    pub total_time_spinning: u64,
    pub locks_taken: u64,
    pub locks_acquired_single_attempt: u64,
}
pub type QfRuntime = QuotientFilterRuntimeData;

#[repr(C)]
pub struct QuotientFilterRuntimeData {
    pub f_info: FileInfo,
    pub auto_resize: u32,
    pub container_resize:
        ::std::option::Option<unsafe extern "C" fn(qf: *mut QF, nslots: u64) -> i64>,
    pub pc_nelts: PartitionedCounter,
    pub pc_ndistinct_elts: PartitionedCounter,
    pub pc_noccupied_slots: PartitionedCounter,
    pub num_locks: u64,
    pub metadata_lock: ::std::os::raw::c_int,
    pub locks: *mut ::std::os::raw::c_int,
    pub wait_times: *mut WaitTimeData,
}

#[repr(C)]
pub struct QuotientFilter {
    pub runtimedata: *mut QfRuntime,
    pub metadata: *mut Qfmetadata,
    pub blocks: *mut Qfblock,
}
pub type QF = QuotientFilter;
#[repr(C)]
pub struct Qfmetadata {
    pub magic_endian_number: u64,
    pub hash_mode: u32,
    pub reserved: u32,
    pub total_size_in_bytes: u64,
    pub seed: u32,
    pub nslots: u64,
    pub xnslots: u64,
    pub key_bits: u64,
    pub value_bits: u64,
    pub key_remainder_bits: u64,
    pub bits_per_slot: u64,
    // pub range: u128,
    pub range: [u64; 2usize],
    pub nblocks: u64,
    pub nelts: u64,
    pub ndistinct_elts: u64,
    pub noccupied_slots: u64,
}
#[repr(C)]
pub struct ClusterData {
    pub start_index: u64,
    pub length: u16,
}

#[repr(C)]
pub struct QuotientFilterIterator {
    pub qf: *const QF,
    pub run: u64,
    pub current: u64,
    pub cur_start_index: u64,
    pub cur_length: u16,
    pub num_clusters: u32,
    pub c_info: *mut ClusterData,
}
pub type QFi = QuotientFilterIterator;
#[repr(C)]
pub struct Qfblock {
    pub offset: u8,
    pub occupieds: [u64; 1usize],
    pub runends: [u64; 1usize],
    // pub slots: __IncompleteArrayField<u8>,
}
#[link(name = "squeakr")]
extern "C" {
    // pub fn squeakr_open(filename: *const std::ffi::c_char) -> *mut QF;
    // pub fn squeakr_iter(squeakr: *mut QF) -> *mut QFi;
    // pub fn squeakr_close(squeakr: *mut QF);
    // pub fn squeakr_next_kmer(squeakr_iter: *mut QFi, kmer: *mut u64) -> i32;

    pub fn get_kmer_size(squeakr: *mut QF) -> u64;
    //mmap versions

    pub fn squeakr_open_mmap(filename: *const std::ffi::c_char) -> *mut QF;
    pub fn squeakr_close_mmap(squeakr: *mut QF);
    pub fn squeakr_clean_iter(squeakr: *mut QFi);
    pub fn squeakr_iter_mmap(squeakr: *const QF) -> *mut QFi;
    pub fn squeakr_next_kmer_mmap(squeakr_iter: *mut QFi, kmer: *mut u64) -> i32;

}

// fn print_kmer(kmer_size: u64, mut kmer_val: u64, f: &mut BufWriter<File>) -> () {
//     let mut kmer_string: Vec<u8> = Vec::with_capacity(kmer_size as usize);
//     for i in (1..=kmer_size).rev() {
//         let base: i64 = ((kmer_val >> ((i*2) -2)) & 3) as i64;
//         let char = match base {
//             0 => b'C',
//             1 => b'A',
//             2 => b'T',
//             3 => b'G',
//             _ => b' '
//         };
//         kmer_string.push(char);
//     }
//     f.write_all(&kmer_string).unwrap();
//     f.write_all(b"\n").unwrap();
// }

// fn main() {
//     // let mut file = File::open("foo.txt").unwrap();
//     let mut file = BufWriter::new(OpenOptions::new().write(true).append(true).open("test.txt").unwrap());

//     let squeakr = unsafe {squeakr_open(CString::new("test.squeakr".to_owned()).unwrap())};

//     if squeakr == 0 as *mut QuotientFilter {
//         panic!("Error reading file");
//     }
//     let kmer_size = unsafe {(*(*squeakr).metadata).key_bits >> 1};
//     let iter = unsafe{squeakr_iter(squeakr)};

//     loop {
//         let kmer = unsafe {squeakr_next_kmer(iter)};
//         if kmer != u64::MAX {
//             print_kmer(kmer_size, kmer, &mut file);
//         } else {
//             break;
//         }
//     }

//     file.flush().unwrap();
//     ()
// }
