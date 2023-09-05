mod squeakr;
pub use squeakr::{
    get_kmer_size, squeakr_clean_iter, squeakr_close_mmap, squeakr_iter_mmap,
    squeakr_next_kmer_mmap, squeakr_open_mmap, QFi, QF,
};
