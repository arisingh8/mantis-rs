#include <stdint.h>
#include <stdlib.h>
// #include <inttypes.h>
// #include <string.h>
// #include <math.h>
#include <sys/mman.h>

#include "gqf.h"
#include "gqf_int.h"
#include "gqf_file.h"


#define NUM_HASH_BITS 24
#define NUM_Q_BITS 16
#define SEED 2038074761

// typedef struct Squeakr Squeakr;
typedef QF Squeakr;
typedef QFi SqueakrIter;
