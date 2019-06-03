#include "../../PQClean/common/aes.h"


/* prototypes for previously extern functions */
extern uint32_t br_dec32le(const unsigned char *src);
extern void br_range_dec32le(uint32_t *v, size_t num, const unsigned char *src);
extern uint32_t br_swap32(uint32_t x);
