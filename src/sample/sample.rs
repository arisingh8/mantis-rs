use crate::squeakr::{
    get_kmer_size, squeakr_clean_iter, squeakr_close_mmap, squeakr_iter_mmap,
    squeakr_next_kmer_mmap, squeakr_open_mmap, QFi, QF,
};
use std::ffi::CString;
use std::{path::PathBuf, string::String};

enum Bases {
    C = 0,
    A = 1,
    T = 2,
    G = 3,
}

fn base_to_u8(base: u8) -> u8 {
    match base {
        b'A' => Bases::A as u8,
        b'C' => Bases::C as u8,
        b'G' => Bases::G as u8,
        b'T' => Bases::T as u8,
        _ => panic!("Invalid base"),
    }
}

pub trait StringBasedSample {
    fn new(sequence: String, sample_id: String, kmer_size: u32) -> Result<Self, ()>
    where
        Self: Sized;

    fn get_sample_id(&self) -> String;

    fn into_raw_sequence(self) -> Vec<u8>;

    fn get_raw_sequence_as_ref(&self) -> &Vec<u8>;

    fn get_kmer_size(&self) -> u16;

    fn close(self) -> ();
}

/// Wrapper Type for string based samples that reveals methods for accessing the underlying sequence
pub trait StringBasedSampleReader: StringBasedSample + Iterator<Item = u64> {}
impl<T> StringBasedSampleReader for T where T: StringBasedSample + Iterator<Item = u64> {}
pub struct StringSampleReader {
    sequence: Vec<u8>,
    sample_id: String,
    current_index: usize,
    current_kmer: u64,
    mask: u64,
}

impl StringBasedSample for StringSampleReader {
    fn new(sequence: String, sample_id: String, kmer_size: u32) -> Result<Self, ()> {
        if sequence.len() < kmer_size as usize {
            return Err(());
        }

        let sequence_bytes = sequence.as_bytes().to_ascii_uppercase();

        // Check if the sequence contains invalid bases (can be improved in necessary using SIMD)
        for i in 0..sequence_bytes.len() {
            if sequence_bytes[i] != b'A'
                && sequence_bytes[i] != b'C'
                && sequence_bytes[i] != b'G'
                && sequence_bytes[i] != b'T'
            {
                return Err(());
            }
        }

        let mut current_kmer: u64 = 0;
        for i in 0..kmer_size - 1 {
            current_kmer = current_kmer << 2;
            current_kmer |= base_to_u8(sequence_bytes[i as usize]) as u64;
        }

        Ok(Self {
            sequence: sequence.into_bytes(),
            sample_id,
            current_index: (kmer_size - 1) as usize,
            current_kmer: (current_kmer & (1 << (2 * kmer_size)) - 1),
            mask: (1 << (2 * kmer_size)) - 1,
        })
    }

    fn close(self) -> () {
        // Do nothing
        ()
    }

    fn get_kmer_size(&self) -> u16 {
        (self.mask.count_ones() >> 1) as u16
    }

    fn get_sample_id(&self) -> String {
        self.sample_id.clone()
    }

    // Consumes self, returning the raw sequence
    fn into_raw_sequence(self) -> Vec<u8> {
        // let sequence = self.sequence;
        self.sequence
    }

    fn get_raw_sequence_as_ref(&self) -> &Vec<u8> {
        &self.sequence
    }
}

impl Iterator for StringSampleReader {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.sequence.len() {
            return None;
        } else {
            let mut next = self.current_kmer << 2;
            next = next | ((base_to_u8(self.sequence[self.current_index]) as u64) & 0b11);
            self.current_kmer = next & self.mask;
            self.current_index += 1;
            Some(self.current_kmer)
        }
    }
}

pub trait FileBasedSampleReader: FileBasedSample + IntoIterator<Item = u64> {}
impl<T> FileBasedSampleReader for T where T: FileBasedSample + IntoIterator<Item = u64> {}

pub trait FileBasedSample {
    type UnderlyingObject;

    fn new(sample_id: String, sample_path: String) -> Result<Self, ()>
    where
        Self: Sized;

    fn open(&mut self) -> Result<(), ()>;

    fn get_sample_id(&self) -> String;

    fn into_raw_pointer(self) -> std::ptr::NonNull<Self::UnderlyingObject>;

    fn get_raw_pointer(&self) -> std::ptr::NonNull<Self::UnderlyingObject>;

    // Returns canonical filename
    fn get_file_name(&self) -> CString;

    fn get_kmer_size(&self) -> Option<u16>;

    fn close(&mut self) -> ();
}

pub struct Squeakr {
    sample_id: String,
    sample_path: CString,
    quotient_filter: *mut QF,
    kmer_size: u16,
}

impl FileBasedSample for Squeakr {
    type UnderlyingObject = QF;
    /// lazily initiates squeakr and iterator
    fn new(sample_id: String, sample_path: String) -> Result<Self, ()> {
        let path = PathBuf::from(sample_path.clone());
        match path.try_exists() {
            Ok(true) => (),
            Ok(false) | Err(_) => return Err(()),
        };
        let c_file_path = match CString::new(sample_path) {
            Ok(c_filename) => c_filename,
            Err(_) => return Err(()),
        };
        Ok(Squeakr {
            sample_id,
            sample_path: c_file_path,
            quotient_filter: std::ptr::null_mut(),
            kmer_size: 0,
        })
    }

    fn open(&mut self) -> Result<(), ()> {
        if self.quotient_filter.is_null() {
            tracing::info!("Opening quotient filter");
            self.quotient_filter = unsafe { squeakr_open_mmap(self.sample_path.as_ptr()) };
            if self.quotient_filter.is_null() {
                // as of right now, squeakr_open_mmap exits on failure, can be changed if necessary in the future
                tracing::info!("Failed to open squeakr");
                return Err(());
            }
            self.kmer_size = unsafe { get_kmer_size(self.quotient_filter) as u16 };
        }
        Ok(())
    }

    fn get_kmer_size(&self) -> Option<u16> {
        if self.kmer_size == 0 {
            None
        } else {
            Some(self.kmer_size as u16)
        }
    }

    fn into_raw_pointer(self) -> std::ptr::NonNull<Self::UnderlyingObject> {
        let ptr = unsafe { std::ptr::NonNull::new_unchecked(self.quotient_filter) };
        std::mem::forget(self);
        ptr
    }

    fn get_raw_pointer(&self) -> std::ptr::NonNull<Self::UnderlyingObject> {
        unsafe { std::ptr::NonNull::new_unchecked(self.quotient_filter) }
    }

    fn close(&mut self) -> () {
        if !self.quotient_filter.is_null() {
            unsafe {
                squeakr_close_mmap(self.quotient_filter);
            }
            self.quotient_filter = std::ptr::null_mut();
        }
    }

    fn get_file_name(&self) -> CString {
        self.sample_path.clone()
    }

    fn get_sample_id(&self) -> String {
        self.sample_id.clone()
    }
}

impl IntoIterator for Squeakr {
    type Item = u64;
    type IntoIter = QFiWrapper;

    // Open must be called before this function
    fn into_iter(mut self) -> Self::IntoIter {
        if self.quotient_filter.is_null() {
            self.open().expect("Failed to open squeakr file");
            panic!("Open must be called before starting to iterate")
        }
        let iter_pointer = unsafe { squeakr_iter_mmap(self.quotient_filter) };
        if iter_pointer.is_null() {
            panic!("Could not get iterator")
        }
        QFiWrapper(iter_pointer)
    }
}

pub struct QFiWrapper(*mut QFi);

impl Iterator for QFiWrapper {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut kmer = 0u64;
        let kmer_ptr = &mut kmer as *mut u64;
        let result = unsafe { squeakr_next_kmer_mmap(self.0 as *mut QFi, kmer_ptr) };
        if result == 0 {
            Some(kmer)
        } else {
            unsafe { squeakr_clean_iter(self.0 as *mut QFi) }
            None
        }
    }
}