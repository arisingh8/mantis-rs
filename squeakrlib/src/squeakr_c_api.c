#include "squeakr_c_api.h"

// // Return the number of kmers in the squeakr
uint64_t get_kmer_size(QF* squeakr) {
    return (squeakr->metadata->key_bits >> 1); // may need to divide by 2
}

// // Writes the size of the squeakr to the size pointer
// Squeakr* squeakr_open(const char* filename) {
//     Squeakr* squeakr = malloc(sizeof(Squeakr));
//     if (qf_deserialize(squeakr, filename) < 1) {
//         free(squeakr);
//         return NULL;
//     }
//     return squeakr;
// }

// QFi* squeakr_iter(QF* squeakr) {
//     QFi* iter = malloc(sizeof(QFi));
//     if (QFI_INVALID == qf_iterator_from_position(squeakr, iter, 0)) {
//         free(iter);
//         return NULL;
//     }
//     return iter;
// }

// void squeakr_close(Squeakr* squeakr) {
//     qf_free(squeakr);
//     free(squeakr);
// }

// uint64_t squeakr_next_kmer(SqueakrIter* squeakr_iter, uint64_t* kmer) {
//     uint64_t value = 0, count = 0;
//     int error = qfi_get_key(squeakr_iter, kmer, &value, &count);
//     if (error == QFI_INVALID || error == QF_INVALID) {
//         // dont seem to be using c_info
//         // if (squeakr_iter->c_info != NULL) {
//         //     free(squeakr_iter->c_info);
//         // }
//         free(squeakr_iter);
//         return -1;
//     }
//     qfi_next(squeakr_iter);
//     return 0;
// }

Squeakr* squeakr_open_mmap(const char* filename) {
    Squeakr* squeakr = malloc(sizeof(Squeakr));
    uint64_t size = 0;
    size = qf_usefile(squeakr, filename, QF_USEFILE_READ_ONLY);
    if (size < 1) {
        free(squeakr);
        return NULL;
    }
    return squeakr;
}

// Create a new iterator for the mmap'd squeakr
SqueakrIter* squeakr_iter_mmap(Squeakr* squeakr) {
    SqueakrIter* iter = malloc(sizeof(SqueakrIter));
    if (qf_iterator_from_position(squeakr, iter, 0) >= 0) {
        qfi_initial_madvise(iter); // might not be necessary
        return iter;
    }
    free(iter);
    return NULL;
}

// returns -1 when iterator is completed
int squeakr_next_kmer_mmap(SqueakrIter* squeakr_iter, uint64_t* kmer) {
    uint64_t value = 0, count = 0;
    int ret = qfi_get_key(squeakr_iter, kmer, &value, &count);
    if (ret == QFI_INVALID || ret == QF_INVALID) {
        // c_info not used
        // if (squeakr_iter->c_info != NULL) {
        //     free(squeakr_iter->c_info);
        // }
        return -1;
    }
    qfi_next_madvise(squeakr_iter);
    return 0;
}

void squeakr_free_iter(SqueakrIter* squeakr_iter) {
    free(squeakr_iter);
}

void squeakr_clean_iter(SqueakrIter* squeakr_iter) {
    qf_closefile(squeakr_iter->qf);
    free(squeakr_iter);
}

void squeakr_close_mmap(Squeakr* squeakr) {
    qf_closefile(squeakr);
    free(squeakr);
}
