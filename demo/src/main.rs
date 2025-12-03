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

        let n = 50;
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
            fmpz_set_si(fmpz_mat_entry(&mat, i, i), 1);
            let sc = Integer::from(&coeffs[i as usize] * &scale);
            fmpz_set_mpz(fmpz_mat_entry(&mat, i, n), sc.as_raw());
        }
        eprintln!("Matrix before LLL:");
        for i in 0..n {
            for j in 0..n+1 {
                fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mat, i, j));
                eprint!("{x} ");
            }
            eprintln!();
        }

        let mut fl = std::mem::MaybeUninit::<fmpz_lll_struct>::uninit();
        fmpz_lll_context_init_default(fl.as_mut_ptr());
        let fl = fl.assume_init();

        let start = std::time::Instant::now();
        fmpz_lll(&mut mat, std::ptr::null_mut(), &fl);
        dbg!(start.elapsed());

        eprintln!("Matrix after LLL:");
        let mut num_solutions = 0;
        for i in 0..n {
            for j in 0..n+1 {
                fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mat, i, j));
                eprint!("{x} ");
            }
            eprintln!();

            if x == 0 {
                let mut s = Integer::from(0);
                for j in 0..n {
                    fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mat, i, j));
                    s += &coeffs[j as usize] * &x;
                }
                assert_eq!(s, 0);
                num_solutions += 1;
            }
        }
        dbg!(num_solutions);

        fmpz_mat_clear(&mut mat);
    }
}
