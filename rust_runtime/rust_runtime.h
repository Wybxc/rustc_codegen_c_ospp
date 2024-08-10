#include <stdbool.h>
#include <stdint.h>

/** cast from unsigned to signed
 * example: `__rust_utos(uint32_t, int32_t, x, INT32_MAX)`
 */
#define __rust_utos(u, s, v, m)                                                \
  ((v) <= (m) ? ((s)v) : ((s)((u)(v) - (u)(m) - 1)))

bool __rust_ckd_add_i8(int8_t x, int8_t y, int8_t *result);
bool __rust_ckd_add_i16(int16_t x, int16_t y, int16_t *result);
bool __rust_ckd_add_i32(int32_t x, int32_t y, int32_t *result);
bool __rust_ckd_add_i64(int64_t x, int64_t y, int64_t *result);
bool __rust_ckd_add_intptr(intptr_t x, intptr_t y, intptr_t *result);
bool __rust_ckd_add_u8(uint8_t x, uint8_t y, uint8_t *result);
bool __rust_ckd_add_u16(uint16_t x, uint16_t y, uint16_t *result);
bool __rust_ckd_add_u32(uint32_t x, uint32_t y, uint32_t *result);
bool __rust_ckd_add_u64(uint64_t x, uint64_t y, uint64_t *result);
bool __rust_ckd_add_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result);
bool __rust_ckd_sub_i8(int8_t x, int8_t y, int8_t *result);
bool __rust_ckd_sub_i16(int16_t x, int16_t y, int16_t *result);
bool __rust_ckd_sub_i32(int32_t x, int32_t y, int32_t *result);
bool __rust_ckd_sub_i64(int64_t x, int64_t y, int64_t *result);
bool __rust_ckd_sub_intptr(intptr_t x, intptr_t y, intptr_t *result);
bool __rust_ckd_sub_u8(uint8_t x, uint8_t y, uint8_t *result);
bool __rust_ckd_sub_u16(uint16_t x, uint16_t y, uint16_t *result);
bool __rust_ckd_sub_u32(uint32_t x, uint32_t y, uint32_t *result);
bool __rust_ckd_sub_u64(uint64_t x, uint64_t y, uint64_t *result);
bool __rust_ckd_sub_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result);
bool __rust_ckd_mul_i8(int8_t x, int8_t y, int8_t *result);
bool __rust_ckd_mul_i16(int16_t x, int16_t y, int16_t *result);
bool __rust_ckd_mul_i32(int32_t x, int32_t y, int32_t *result);
bool __rust_ckd_mul_i64(int64_t x, int64_t y, int64_t *result);
bool __rust_ckd_mul_intptr(intptr_t x, intptr_t y, intptr_t *result);
bool __rust_ckd_mul_u8(uint8_t x, uint8_t y, uint8_t *result);
bool __rust_ckd_mul_u16(uint16_t x, uint16_t y, uint16_t *result);
bool __rust_ckd_mul_u32(uint32_t x, uint32_t y, uint32_t *result);
bool __rust_ckd_mul_u64(uint64_t x, uint64_t y, uint64_t *result);
bool __rust_ckd_mul_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result);
bool __rust_ckd_div_i8(int8_t x, int8_t y, int8_t *result);
bool __rust_ckd_div_i16(int16_t x, int16_t y, int16_t *result);
bool __rust_ckd_div_i32(int32_t x, int32_t y, int32_t *result);
bool __rust_ckd_div_i64(int64_t x, int64_t y, int64_t *result);
bool __rust_ckd_div_intptr(intptr_t x, intptr_t y, intptr_t *result);
bool __rust_ckd_div_u8(uint8_t x, uint8_t y, uint8_t *result);
bool __rust_ckd_div_u16(uint16_t x, uint16_t y, uint16_t *result);
bool __rust_ckd_div_u32(uint32_t x, uint32_t y, uint32_t *result);
bool __rust_ckd_div_u64(uint64_t x, uint64_t y, uint64_t *result);
bool __rust_ckd_div_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result);
bool __rust_ckd_rem_i8(int8_t x, int8_t y, int8_t *result);
bool __rust_ckd_rem_i16(int16_t x, int16_t y, int16_t *result);
bool __rust_ckd_rem_i32(int32_t x, int32_t y, int32_t *result);
bool __rust_ckd_rem_i64(int64_t x, int64_t y, int64_t *result);
bool __rust_ckd_rem_intptr(intptr_t x, intptr_t y, intptr_t *result);
bool __rust_ckd_rem_u8(uint8_t x, uint8_t y, uint8_t *result);
bool __rust_ckd_rem_u16(uint16_t x, uint16_t y, uint16_t *result);
bool __rust_ckd_rem_u32(uint32_t x, uint32_t y, uint32_t *result);
bool __rust_ckd_rem_u64(uint64_t x, uint64_t y, uint64_t *result);
bool __rust_ckd_rem_uintptr(uintptr_t x, uintptr_t y, uintptr_t *result);
