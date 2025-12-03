#![allow(non_camel_case_types)]

use libc::c_long;
use gmp_mpfr_sys::gmp::mpz_t;

// In FLINT, slong is either long or long long depending on FLINT_LONG_LONG
// For most platforms, it's just long

pub type slong = c_long;

pub type fmpz = slong;

pub type fmpz_t = [fmpz; 1];

unsafe extern "C" {
    pub fn fmpz_init(f: *mut fmpz_t);
    pub fn fmpz_clear(f: *mut fmpz_t);

    pub fn fmpz_set_si(f: *mut fmpz_t, val: slong);
    /// Warning: Result is undefined if f does not fit into a slong
    pub fn fmpz_get_si(f: *const fmpz_t) -> slong;

    pub fn fmpz_set_mpz(f: *mut fmpz_t, x: *const mpz_t);
    pub fn fmpz_get_mpz(x: *mut mpz_t, f: *const fmpz_t);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gmp_mpfr_sys::gmp;

    #[test]
    fn test_fmpz_roundtrip() {
        unsafe {
            let mut x: fmpz_t = [0];
            fmpz_init(&mut x);
            fmpz_set_si(&mut x, 42);
            let result = fmpz_get_si(&x);
            fmpz_clear(&mut x);
            assert_eq!(result, 42);

            let mut x: fmpz_t = [0];
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
            let mut f: fmpz_t = [0];
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
