#![allow(non_camel_case_types)]

use libc::c_long;

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
