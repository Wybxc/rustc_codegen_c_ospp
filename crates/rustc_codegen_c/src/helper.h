#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

/** cast from unsigned to signed
 * example: `__rust_utos(uint32_t, int32_t, x, INT32_MAX)`
 */
#define __rust_utos(u, s, v, m)                                                \
  ((v) <= (m) ? ((s)v) : ((s)((u)(v) - (u)(m) - 1)))

inline int32_t __rust_checked_add_i32(int32_t x, int32_t y, bool *overflow) {
  int64_t sum = (int64_t)x + (int64_t)y;
  *overflow = sum < x || sum < y;
  return sum;
}

inline int32_t __rust_checked_sub_i32(int32_t x, int32_t y, bool *overflow) {
  int64_t diff = (int64_t)x - (int64_t)y;
  *overflow = diff > x || diff > y;
  return diff;
}
