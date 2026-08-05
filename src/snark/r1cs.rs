use crate::snark::witness::{COLS, L, OUT, R, ROWS, SUBGROUP_ORDER, WITNESS};
use cryptography::exercises::{ec_point::Field, finite_field::Fp};

pub fn validate() {
    let l: [[Fp; COLS]; ROWS] = L.map(|row| row.map(|x| gf(x)));
    let r: [[Fp; COLS]; ROWS] = R.map(|row| row.map(|y| gf(y)));
    let out: [[Fp; COLS]; ROWS] = OUT.map(|row| row.map(|z| gf(z)));
    let witness = WITNESS.map(|x| gf(x));

    // L will be multiplied by G₁
    // R will be multiplied by G₂
    let l_mul = witness_mul(&l, &witness);
    let r_mul = witness_mul(&r, &witness);
    let out_mul = witness_mul(&out, &witness);

    // The hadamard product will contain a pairing of G₁ • G₂
    let product = hadamard_product(&l_mul, &r_mul);

    assert_eq!(product, out_mul);
    println!("R1CS computed correctly!");
}

fn witness_mul<T>(vector: &[[T; COLS]; ROWS], witness: &[T; COLS]) -> [T; ROWS]
where
    T: Field,
{
    let mut result: [T; ROWS] = [T::zero(SUBGROUP_ORDER); ROWS];

    for i in 0..result.len() {
        let sum = &mut result[i];

        for j in 0..COLS {
            *sum = *sum + (vector[i][j] * witness[j]);
        }
    }

    result
}

fn hadamard_product<T>(l_matrix: &[T; ROWS], r_matrix: &[T; ROWS]) -> [T; ROWS]
where
    T: Field,
{
    let result: [T; ROWS] = std::array::from_fn(|i| l_matrix[i] * r_matrix[i]);

    result
}

pub fn gf(value: i64) -> Fp {
    Fp::new(value, SUBGROUP_ORDER).unwrap()
}
