#![cfg(target_feature = "atomics")]

#[cfg(test)]
use stdsimd_test::assert_instr;
#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test;

extern "C" {
    #[link_name = "llvm.wasm.atomic.wait.i32"]
    fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32;
    #[link_name = "llvm.wasm.atomic.wait.i64"]
    fn llvm_atomic_wait_i64(ptr: *mut i64, exp: i64, timeout: i64) -> i32;
    #[link_name = "llvm.wasm.atomic.notify"]
    fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> i32;
}

#[inline]
#[cfg_attr(test, assert_instr("i32.atomic.wait"))]
pub unsafe fn wait_i32(ptr: *mut i32, expression: i32, timeout_ns: i64) -> i32 {
    llvm_atomic_wait_i32(ptr, expression, timeout_ns)
}

#[inline]
#[cfg_attr(test, assert_instr("i64.atomic.wait"))]
pub unsafe fn wait_i64(ptr: *mut i64, expression: i64, timeout_ns: i64) -> i32 {
    llvm_atomic_wait_i64(ptr, expression, timeout_ns)
}

#[inline]
#[cfg_attr(test, assert_instr("atomic.wake"))]
pub unsafe fn wake(ptr: *mut i32, waiters: i32) -> i32 {
    llvm_atomic_notify(ptr, waiters)
}
