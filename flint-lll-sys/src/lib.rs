#![allow(non_camel_case_types)]

use libc::c_long;
use gmp_mpfr_sys::gmp::mpz_t;

// In FLINT, slong is either long or long long depending on FLINT_LONG_LONG
// For most platforms, it's just long
pub type slong = c_long;

pub type fmpz = slong;

#[repr(C)]
pub struct fmpz_mat_struct {
    pub entries: *mut fmpz,
    pub r: slong,
    pub c: slong,
    pub stride: slong,
}

// LLL types
#[repr(C)]
pub enum rep_type {
    GRAM = 0,
    Z_BASIS = 1,
}

#[repr(C)]
pub enum gram_type {
    APPROX = 0,
    EXACT = 1,
}

#[repr(C)]
pub struct fmpz_lll_struct {
    pub delta: f64,
    pub eta: f64,
    pub rt: rep_type,
    pub gt: gram_type,
}

unsafe extern "C" {
    // fmpz functions
    pub fn fmpz_init(f: *mut fmpz);
    pub fn fmpz_clear(f: *mut fmpz);

    pub fn fmpz_set_si(f: *mut fmpz, val: slong);
    pub fn fmpz_get_si(f: *const fmpz) -> slong; // undefined if f does not fit into a slong

    pub fn fmpz_set_mpz(f: *mut fmpz, x: *const mpz_t);
    pub fn fmpz_get_mpz(x: *mut mpz_t, f: *const fmpz);

    // fmpz_mat functions
    pub fn fmpz_mat_init(mat: *mut fmpz_mat_struct, rows: slong, cols: slong);
    pub fn fmpz_mat_clear(mat: *mut fmpz_mat_struct);

    pub fn fmpz_mat_entry(mat: *const fmpz_mat_struct, i: slong, j: slong) -> *mut fmpz;

    // fmpz_lll functions
    pub fn fmpz_lll_context_init_default(fl: *mut fmpz_lll_struct);
    pub fn fmpz_lll(B: *mut fmpz_mat_struct, U: *mut fmpz_mat_struct, fl: *const fmpz_lll_struct);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gmp_mpfr_sys::gmp;

    #[test]
    fn test_fmpz_roundtrip() {
        unsafe {
            let mut x: fmpz = 0;
            fmpz_init(&mut x);
            fmpz_set_si(&mut x, 42);
            let result = fmpz_get_si(&x);
            fmpz_clear(&mut x);
            assert_eq!(result, 42);

            let mut x: fmpz = 0;
            fmpz_init(&mut x);
            fmpz_set_si(&mut x, -12345);
            let result = fmpz_get_si(&x);
            fmpz_clear(&mut x);
            assert_eq!(result, -12345);
        }
    }

    #[test]
    fn test_fmpz_mpz_roundtrip() {
        unsafe {
            // Create a large GMP integer (larger than fits in slong)
            let mut z = std::mem::MaybeUninit::<mpz_t>::uninit();
            gmp::mpz_init(z.as_mut_ptr());
            let mut z = z.assume_init();

            // Set to a large value: 2^100
            gmp::mpz_ui_pow_ui(&mut z, 2, 100);

            // Convert to fmpz
            let mut f: fmpz = 0;
            fmpz_init(&mut f);
            fmpz_set_mpz(&mut f, &z);

            // Convert back to mpz
            let mut z2 = std::mem::MaybeUninit::<mpz_t>::uninit();
            gmp::mpz_init(z2.as_mut_ptr());
            let mut z2 = z2.assume_init();
            fmpz_get_mpz(&mut z2, &f);

            // Verify they're equal
            let cmp = gmp::mpz_cmp(&z, &z2);
            assert_eq!(cmp, 0);

            // Clean up
            fmpz_clear(&mut f);
            gmp::mpz_clear(&mut z);
            gmp::mpz_clear(&mut z2);
        }
    }
}
