use rand::prelude::*;
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
        dbg!(&x);

        let mut rng = rand::rng();

        let n = 10;
        let coeffs: Vec<Integer> = (0..n).map(|_| {
            let mut x = Integer::from(0);
            for _ in 0..100 {
                x *= 10;
                x += rng.random_range(0..10);
            }
            x
        }).collect();

        let scale: Integer = coeffs.iter().map(|x| x.clone().abs()).max().unwrap() * 10;

        let mut mat = std::mem::MaybeUninit::<fmpz_mat_struct>::uninit();
        fmpz_mat_init(mat.as_mut_ptr(), n, n + 1);
        let mut mat = mat.assume_init();

        for i in 0..n {
            fmpz_set_si(fmpz_mat_entry(&mut mat, i, i), 1);
            let sc = Integer::from(&coeffs[i as usize] * &scale);
            fmpz_set_mpz(fmpz_mat_entry(&mut mat, i, n), sc.as_raw());
        }
        for i in 0..n {
            for j in 0..n+1 {
                fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mut mat, i, j));
                eprint!("{x} ");
            }
            eprintln!();
        }
    }
}
