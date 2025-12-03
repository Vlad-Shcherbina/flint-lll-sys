use std::iter::zip;
use rand::prelude::*;
use rug::Integer;
use flint_lll_sys::*;

fn find_integer_relations(coeffs: &[Integer]) -> Vec<Vec<Integer>> {
    unsafe {
        let n = slong::try_from(coeffs.len()).unwrap();
        let scale: Integer = coeffs.iter().map(|x| x.clone().abs()).max().unwrap() * 10;

        let mut mat = std::mem::MaybeUninit::<fmpz_mat_struct>::uninit();
        fmpz_mat_init(mat.as_mut_ptr(), n, n + 1);
        let mut mat = mat.assume_init();

        for i in 0..n {
            fmpz_set_si(fmpz_mat_entry(&mat, i, i), 1);
            let sc = Integer::from(&coeffs[i as usize] * &scale);
            fmpz_set_mpz(fmpz_mat_entry(&mat, i, n), sc.as_raw());
        }

        // eprintln!("Matrix before LLL:");
        // for i in 0..n {
        //     for j in 0..n+1 {
        //         fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mat, i, j));
        //         eprint!("{x} ");
        //     }
        //     eprintln!();
        // }

        let mut fl = std::mem::MaybeUninit::<fmpz_lll_struct>::uninit();
        fmpz_lll_context_init_default(fl.as_mut_ptr());
        let fl = fl.assume_init();

        fmpz_lll(&mut mat, std::ptr::null_mut(), &fl);

        let mut sols = vec![];
        for i in 0..n {
            let mut x = Integer::from(0);
            fmpz_get_mpz(x.as_raw_mut(), fmpz_mat_entry(&mat, i, n));
            if x == 0 {
                let mut sol = vec![Integer::from(0); n as usize];
                for j in 0..n {
                    fmpz_get_mpz(sol[j as usize].as_raw_mut(), fmpz_mat_entry(&mat, i, j));
                }
                let s: Integer = zip(coeffs, &sol).map(|(c, x)| c * x).sum();
                assert_eq!(s, 0);
                sols.push(sol);
            }
        }
        fmpz_mat_clear(&mut mat);

        sols
    }
}

fn main() {
    let mut rng = rand::rng();

    let n = 50;
    let coeffs: Vec<Integer> = (0..n).map(|_| {
        let mut x = Integer::from(0);
        for _ in 0..100 {
            x *= 10;
            x += rng.random_range(0..10);
        }
        if rng.random_bool(0.5) { x } else { -x }
    }).collect();
    dbg!(&coeffs);

    let start = std::time::Instant::now();
    let sols = find_integer_relations(&coeffs);
    dbg!(start.elapsed());

    for sol in &sols {
        eprintln!("{sol:?}");
    }
    dbg!(sols.len());
}
