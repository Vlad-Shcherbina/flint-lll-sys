#![allow(non_camel_case_types)]

use libc::c_long;
use gmp_mpfr_sys::gmp::mpz_t;

// In FLINT, slong is either long or long long depending on FLINT_LONG_LONG
// For most platforms, it's just long
pub type slong = c_long;

pub type fmpz = slong;
// In C: typedef fmpz fmpz_t[1];
// The array decays to a pointer when passed to functions.
// We add this type alias to preserve syntactic similarity between
// C function headers and Rust declarations.
// But this alias should only be used in function arg types!
type fmpz_t = *mut fmpz;

#[repr(C)]
pub struct fmpz_mat_struct {
    pub entries: *mut fmpz,
    pub r: slong,
    pub c: slong,
    pub stride: slong,
}

// In C: typedef fmpz_mat_struct fmpz_mat_t[1];
// Same pattern as fmpz_t - represents the pointer after array decay.
type fmpz_mat_t = *mut fmpz_mat_struct;

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

// In C: typedef fmpz_lll_struct fmpz_lll_t[1];
// Same pattern as fmpz_t and fmpz_mat_t
type fmpz_lll_t = *mut fmpz_lll_struct;

unsafe extern "C" {
    // fmpz functions
    pub fn fmpz_init(f: fmpz_t);
    pub fn fmpz_clear(f: fmpz_t);

    pub fn fmpz_set_si(f: fmpz_t, val: slong);
    pub fn fmpz_get_si(f: fmpz_t) -> slong; // undefined if f does not fit into a slong

    pub fn fmpz_set_mpz(f: fmpz_t, x: *const mpz_t);
    pub fn fmpz_get_mpz(x: *mut mpz_t, f: fmpz_t);

    // fmpz_mat functions
    pub fn fmpz_mat_init(mat: fmpz_mat_t, rows: slong, cols: slong);
    pub fn fmpz_mat_clear(mat: fmpz_mat_t);

    pub fn fmpz_mat_entry(mat: fmpz_mat_t, i: slong, j: slong) -> *mut fmpz;

    // fmpz_lll functions
    pub fn fmpz_lll_context_init_default(fl: fmpz_lll_t);
    pub fn fmpz_lll(B: fmpz_mat_t, U: fmpz_mat_t, fl: fmpz_lll_t);
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
            let result = fmpz_get_si(&mut x);
            fmpz_clear(&mut x);
            assert_eq!(result, 42);

            let mut x: fmpz = 0;
            fmpz_init(&mut x);
            fmpz_set_si(&mut x, -12345);
            let result = fmpz_get_si(&mut x);
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
            fmpz_get_mpz(&mut z2, &mut f);

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
