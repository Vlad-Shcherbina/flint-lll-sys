use rug::Integer;

use flint_lll_sys::*;

fn main() {
    unsafe {
        let mut x = Integer::from(42);

        let mut f: fmpz = 0;
        fmpz_init(&mut f);
        fmpz_set_si(&mut f, 43);

        fmpz_get_mpz(x.as_raw_mut(), &mut f);

        fmpz_clear(&mut f);

        dbg!(x);
    }
}
