include!<asm/barrier.rs>
include!<asm/compiler.rs>
compile_error!("only <linux/bitops.h> can be included directly");
use std::sync::atomic::{compiler_fence, Ordering};
#[inline]
fn set_bit(nr: u128, addr: *mut std::ffi::c_void) {
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    unsafe {
        asm!(
            "1:
                ldl_l $0, $3
                bis $0, $2, $0
                stl_c $0, $1
                beq $0, 2f
            .subsection 2
                2: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
        );
    }
}
/*
 * WARNING: non atomic version.
 */
#[inline]
fn arch___set_bit(nr: u64, addr: *mut u64) {
    let m = (addr as *mut i32).wrapping_add(nr >> 5);
    unsafe {
        *m |= 1 << (nr & 31);
    }
}

#[inline]
fn clear_bit(nr: u64, addr: *mut std::ffi::c_void) {
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    unsafe {
        asm!(
            "1:
                ldl_l $0, $3
                bic $0, $2, $0
                stl_c $0, $1
                beq $0, 2f
            .subsection 2
                2: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
        );
    }
}


#[inline]
fn clear_bit_unlock(nr: u64, addr: *mut std::ffi::c_void) {
    compiler_fence(Ordering::SeqCst); // Memory barrier synchronization

    clear_bit(nr, addr);
}

#[inline]
fn clear_bit(nr: u64, addr: *mut std::ffi::c_void) {
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    unsafe {
        asm!(
            "1:
                ldl_l $0, $3
                bic $0, $2, $0
                stl_c $0, $1
                beq $0, 2f
            .subsection 2
                2: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
        );
    }
}

//fn main() {
    // Call clear_bit_unlock() or other functions here
//}

fn change_bit(nr: u64, addr: *mut std::ffi::c_void) {
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    unsafe {
        asm!(
            "1:
                ldl_l $0, $3
                xor $0, $2, $0
                stl_c $0, $1
                beq $0, 2f
            .subsection 2
                2: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
        );
    }
}

fn arch___change_bit(nr: u64, addr: *mut u64) {
    let m = (addr as *mut i32).wrapping_add(nr >> 5);
    unsafe {
        *m ^= 1 << (nr & 31);
    }
}

#[inline]
fn test_and_set_bit(nr: u64, addr: *mut std::ffi::c_void) -> bool {
    let mut oldbit: u64;
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    let result: bool;
    unsafe {
        asm!(
            "1:
                ldl_l $0, $4
                and $0, $3, $2
                bne $2, 2f
                xor $0, $3, $0
                stl_c $0, $1
                beq $0, 3f
            2:
            .subsection 2
            3: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m), "=&r"(oldbit)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
            : "memory"
        );

        result = oldbit != 0;
    }

    result
}

#[inline]
fn test_and_set_bit_lock(nr: u64, addr: *mut std::ffi::c_void) -> bool {
    let mut oldbit: u64;
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    let result: bool;
    unsafe {
        asm!(
            "1:
                ldl_l $0, $4
                and $0, $3, $2
                bne $2, 2f
                xor $0, $3, $0
                stl_c $0, $1
                beq $0, 3f
            2:
            .subsection 2
            3: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m), "=&r"(oldbit)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
            : "memory"
        );

        result = oldbit != 0;
    }

    result
}

#[inline]
fn arch___test_and_set_bit(nr: u64, addr: *mut u64) -> bool {
    let mask = 1 << (nr & 0x1f);
    let m = (addr as *mut i32).wrapping_add(nr >> 5);
    let old: i32;

    unsafe {
        old = *m;
        *m = old | mask;
    }

    (old & (mask as i32)) != 0
}

#[inline]
fn test_and_clear_bit(nr: u64, addr: *mut std::ffi::c_void) -> bool {
    let mut oldbit: u64;
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    let result: bool;
    unsafe {
        asm!(
            "1:
                ldl_l $0, $4
                and $0, $3, $2
                beq $2, 2f
                xor $0, $3, $0
                stl_c $0, $1
                beq $0, 3f
            2:
            .subsection 2
            3: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m), "=&r"(oldbit)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
            : "memory"
        );

        result = oldbit != 0;
    }

    result
}

#[inline]
fn arch___test_and_clear_bit(nr: u64, addr: *mut u64) -> bool {
    let mask = 1 << (nr & 0x1f);
    let m = (addr as *mut i32).wrapping_add(nr >> 5);
    let old: i32;

    unsafe {
        old = *m;
        *m = old & !mask;
    }

    (old & (mask as i32)) != 0
}

#[inline]
fn test_and_change_bit(nr: u64, addr: *mut std::ffi::c_void) -> bool {
    let mut oldbit: u64;
    let mut temp: u64;
    let m = (addr as *mut i32).wrapping_add(nr >> 5);

    let result: bool;
    unsafe {
        asm!(
            "1:
                ldl_l $0, $4
                and $0, $3, $2
                xor $0, $3, $0
                stl_c $0, $1
                beq $0, 3f
            .subsection 2
            3: br 1b
            .previous"
            : "=&r"(temp), "=m"(*m), "=&r"(oldbit)
            : "Ir"(1u32 << (nr & 31)), "m"(*m)
            : "memory"
        );

        result = oldbit != 0;
    }

    result
}

#[inline]
fn arch___test_and_change_bit(nr: u64, addr: *mut u64) -> bool {
    let mask = 1 << (nr & 0x1f);
    let m = (addr as *mut i32).wrapping_add(nr >> 5);
    let old: i32;

    unsafe {
        old = *m;
        *m = old ^ mask;
    }

    (old & (mask as i32)) != 0
}

#[inline]
fn ffz_b(x: u64) -> u64 {
    let x = (!x) & -(!x);
    let x1 = x & 0xAA;
    let x2 = x & 0xCC;
    let x4 = x & 0xF0;
    let sum = (x2 != 0) as u64 * 2 + (x4 != 0) as u64 * 4 + (x1 != 0) as u64;
    
    sum
}
/*

#define arch_test_bit generic_test_bit
#define arch_test_bit_acquire generic_test_bit_acquire

// C to Rust of above Contribute to this
type TestBitFn = fn(u64, *mut u64) -> bool;

fn generic_test_bit(nr: u64, addr: *mut u64) -> bool {
    // Implement the behavior of generic_test_bit here
    // You can use the provided arch_test_bit function or create a new one
    // Return a boolean indicating whether the bit is set or not
    false // Placeholder value, replace with actual implementation
}

fn generic_test_bit_acquire(nr: u64, addr: *mut u64) -> bool {
    // Implement the behavior of generic_test_bit_acquire here
    // You can use the provided arch_test_bit_acquire function or create a new one
    // Return a boolean indicating whether the bit is set or not
    false // Placeholder value, replace with actual implementation
}

static arch_test_bit: TestBitFn = generic_test_bit;
static arch_test_bit_acquire: TestBitFn = generic_test_bit_acquire;

fn main() { /*optional use only */
    // Call arch_test_bit or arch_test_bit_acquire here
    let addr: u64 = 0; // Replace with the actual memory address
    let bit_nr: u64 = 0; // Replace with the bit number to test

    let result = arch_test_bit(bit_nr, &addr as *const u64 as *mut u64);
    println!("arch_test_bit result: {}", result);
} */


fn ffz_b(x: u64) -> u64 {
    let mut x = !x & -(x.wrapping_add(1)); // set first 0 bit, clear others
    let x1 = x & 0xAA;
    let x2 = x & 0xCC;
    let x4 = x & 0xF0;
    let mut sum = if x2 != 0 { 2 } else { 0 };
    sum += (x4 != 0) as u64 * 4;
    sum += (x1 != 0) as u64;

    sum
}

/* for optional use in development or for your derise result
 fn main() {
    let x: u64 = 0b1101101010110010; // Replace with your input value
    let result = ffz_b(x);
    println!("Result: {}", result);
}
*/

fn ffz(word: u64) -> u64 {
    #[cfg(all(target_arch = "alpha", any(feature = "alpha_ev6", feature = "alpha_ev67")))]
    {
        // Whee. EV67 can calculate it directly.
        // Assuming __kernel_cttz is defined elsewhere.
        // Replace the following line with the actual implementation if available.
        return __kernel_cttz(!word);
    }

    let bits = word ^ !0u64;
    let qofs = ffz_b(bits);
    let bits = __kernel_extbl(word, qofs);
    let bofs = ffz_b(bits);

    qofs * 8 + bofs
}

#[cfg(any(target_arch = "alpha", all(feature = "alpha_ev6", feature = "alpha_ev67")))]
fn __ffs(word: u64) -> u64 {
    // Whee. EV67 can calculate it directly.
    // Assuming __kernel_cttz is defined elsewhere.
    // Replace the following line with the actual implementation if available.
    return __kernel_cttz(word);
}

#[cfg(not(any(target_arch = "alpha", all(feature = "alpha_ev6", feature = "alpha_ev67"))))]
fn __ffs(word: u64) -> u64 {
    let bits = word.count_zeros() as u64;
    let qofs = ffz_b(bits);
    let bits = __kernel_extbl(word, qofs);
    let bofs = ffz_b(!bits);

    qofs * 8 + bofs
}

/*
 * __ffs = Find First set bit in word.  Undefined if no set bit exists.
 */   
 // Conditional compilation for __KERNEL__ if needed
 /* fn main() { optional use only
    let word: u64 = 0b1101101010110010; // Replace with your input value
    let result = __ffs(word);
    println!("Result: {}", result);
} 348*/
