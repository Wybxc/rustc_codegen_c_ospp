#ifndef __RUST_CHECKED_H
#define __RUST_CHECKED_H

#include <stdbool.h>
#include <stdint.h>

bool __rust_ckd_add_i8(int8_t x, int8_t y, int8_t *result) {
  if ((x > 0 && y > INT8_MAX - x) || (x < 0 && y < INT8_MIN - x))
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_i16(int16_t x, int16_t y, int16_t *result) {
  if ((x > 0 && y > INT16_MAX - x) || (x < 0 && y < INT16_MIN - x))
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_i32(int32_t x, int32_t y, int32_t *result) {
  if ((x > 0 && y > INT32_MAX - x) || (x < 0 && y < INT32_MIN - x))
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_i64(int64_t x, int64_t y, int64_t *result) {
  if ((x > 0 && y > INT64_MAX - x) || (x < 0 && y < INT64_MIN - x))
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_intptr(intptr_t x, intptr_t y, intptr_t *result) {
  if ((x > 0 && y > INTPTR_MAX - x) || (x < 0 && y < INTPTR_MIN - x))
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_u8(uint8_t x, uint8_t y, uint8_t *result) {
  if (y > UINT8_MAX - x)
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_u16(uint16_t x, uint16_t y, uint16_t *result) {
  if (y > UINT16_MAX - x)
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_u32(uint32_t x, uint32_t y, uint32_t *result) {
  if (y > UINT32_MAX - x)
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_u64(uint64_t x, uint64_t y, uint64_t *result) {
  if (y > UINT64_MAX - x)
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_add_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result) {
  if (y > UINTPTR_MAX - x)
    return true;
  *result = x + y;
  return false;
}

bool __rust_ckd_sub_i8(int8_t x, int8_t y, int8_t *result) {
  if ((y < 0 && x > INT8_MAX + y) || (y > 0 && x < INT8_MIN + y))
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_i16(int16_t x, int16_t y, int16_t *result) {
  if ((y < 0 && x > INT16_MAX + y) || (y > 0 && x < INT16_MIN + y))
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_i32(int32_t x, int32_t y, int32_t *result) {
  if ((y < 0 && x > INT32_MAX + y) || (y > 0 && x < INT32_MIN + y))
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_i64(int64_t x, int64_t y, int64_t *result) {
  if ((y < 0 && x > INT64_MAX + y) || (y > 0 && x < INT64_MIN + y))
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_intptr(intptr_t x, intptr_t y, intptr_t *result) {
  if ((y < 0 && x > INTPTR_MAX + y) || (y > 0 && x < INTPTR_MIN + y))
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_u8(uint8_t x, uint8_t y, uint8_t *result) {
  if (x < y)
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_u16(uint16_t x, uint16_t y, uint16_t *result) {
  if (x < y)
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_u32(uint32_t x, uint32_t y, uint32_t *result) {
  if (x < y)
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_u64(uint64_t x, uint64_t y, uint64_t *result) {
  if (x < y)
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_sub_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result) {
  if (x < y)
    return true;
  *result = x - y;
  return false;
}

bool __rust_ckd_mul_i8(int8_t x, int8_t y, int8_t *result) {
  int16_t tmp = (int16_t)x * (int16_t)y;
  if (tmp > INT8_MAX || tmp < INT8_MIN)
    return true;
  *result = (int8_t)tmp;
  return false;
}

bool __rust_ckd_mul_i16(int16_t x, int16_t y, int16_t *result) {
  int32_t tmp = (int32_t)x * (int32_t)y;
  if (tmp > INT16_MAX || tmp < INT16_MIN)
    return true;
  *result = (int16_t)tmp;
  return false;
}

bool __rust_ckd_mul_i32(int32_t x, int32_t y, int32_t *result) {
  int64_t tmp = (int64_t)x * (int64_t)y;
  if (tmp > INT32_MAX || tmp < INT32_MIN)
    return true;
  *result = (int32_t)tmp;
  return false;
}

bool __rust_ckd_mul_i64(int64_t x, int64_t y, int64_t *result) {
  if (x > 0) {
    if (y > 0 && x > INT64_MAX / y)
      return true;
    if (y < 0 && y < INT64_MIN / x)
      return true;
  }
  if (x < 0) {
    if (y > 0 && x < INT64_MIN / y)
      return true;
    if (y < 0 && x < INT64_MAX / y)
      return true;
  }
  *result = x * y;
  return false;
}

bool __rust_ckd_mul_intptr(intptr_t x, intptr_t y, intptr_t *result) {
  if (x > 0) {
    if (y > 0 && x > INTPTR_MAX / y)
      return true;
    if (y < 0 && y < INTPTR_MIN / x)
      return true;
  }
  if (x < 0) {
    if (y > 0 && x < INTPTR_MIN / y)
      return true;
    if (y < 0 && x < INTPTR_MAX / y)
      return true;
  }
  *result = x * y;
  return false;
}

bool __rust_ckd_mul_u8(uint8_t x, uint8_t y, uint8_t *result) {
  uint16_t tmp = (uint16_t)x * (uint16_t)y;
  if (tmp > UINT8_MAX)
    return true;
  *result = (uint8_t)tmp;
  return false;
}

bool __rust_ckd_mul_u16(uint16_t x, uint16_t y, uint16_t *result) {
  uint32_t tmp = (uint32_t)x * (uint32_t)y;
  if (tmp > UINT16_MAX)
    return true;
  *result = (uint16_t)tmp;
  return false;
}

bool __rust_ckd_mul_u32(uint32_t x, uint32_t y, uint32_t *result) {
  uint64_t tmp = (uint64_t)x * (uint64_t)y;
  if (tmp > UINT32_MAX)
    return true;
  *result = (uint32_t)tmp;
  return false;
}

bool __rust_ckd_mul_u64(uint64_t x, uint64_t y, uint64_t *result) {
  if (x == 0) {
    *result = 0;
    return false;
  }
  if (y > UINT64_MAX / x)
    return true;
  *result = x * y;
  return false;
}

bool __rust_ckd_mul_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result) {
  if (x == 0) {
    *result = 0;
    return false;
  }
  if (y > UINTPTR_MAX / x)
    return true;
  *result = x * y;
  return false;
}

bool __rust_ckd_div_i8(int8_t x, int8_t y, int8_t *result) {
  if (y == 0 || (x == INT8_MIN && y == -1))
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_i16(int16_t x, int16_t y, int16_t *result) {
  if (y == 0 || (x == INT16_MIN && y == -1))
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_i32(int32_t x, int32_t y, int32_t *result) {
  if (y == 0 || (x == INT32_MIN && y == -1))
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_i64(int64_t x, int64_t y, int64_t *result) {
  if (y == 0 || (x == INT64_MIN && y == -1))
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_intptr(intptr_t x, intptr_t y, intptr_t *result) {
  if (y == 0 || (x == INTPTR_MIN && y == -1))
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_u8(uint8_t x, uint8_t y, uint8_t *result) {
  if (y == 0)
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_u16(uint16_t x, uint16_t y, uint16_t *result) {
  if (y == 0)
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_u32(uint32_t x, uint32_t y, uint32_t *result) {
  if (y == 0)
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_u64(uint64_t x, uint64_t y, uint64_t *result) {
  if (y == 0)
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_div_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result) {
  if (y == 0)
    return true;
  *result = x / y;
  return false;
}

bool __rust_ckd_rem_i8(int8_t x, int8_t y, int8_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_i16(int16_t x, int16_t y, int16_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_i32(int32_t x, int32_t y, int32_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_i64(int64_t x, int64_t y, int64_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_intptr(intptr_t x, intptr_t y, intptr_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_u8(uint8_t x, uint8_t y, uint8_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_u16(uint16_t x, uint16_t y, uint16_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_u32(uint32_t x, uint32_t y, uint32_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_u64(uint64_t x, uint64_t y, uint64_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

bool __rust_ckd_rem_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result) {
  if (y == 0)
    return true;
  *result = x % y;
  return false;
}

#endif