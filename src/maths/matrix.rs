// step 1: take a 2D vector and map it to a vector of field elements
// For MODULUS = 157
// Curve: y² = x³ - 1x - 1
// ----------------------------------------------------------
// No. of points #E(Fp): 158
// Potential subgroups order `r` for G1: [2, 79]
// embedding degrees: {2: Some(1), 79: Some(2)}

// G1 • (39, 54) — 𝔽157
// G2 • (31 + 153α, 5 + 59α) — 𝔽157²

use std::ops;

use cryptography::exercises::finite_field::Fp;

const MODULUS: u32 = 79;

static L: [[i64; 11]; 5] = [
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

static R: [[i64; 11]; 5] = [
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

static OUT: [[i64; 11]; 5] = [
    [0, 0, -3, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, -3, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, -3, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, -3, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

static WITNESS: [i64; 11] = [1, 47, 78, 73, 72, 49, 15, 76, 44, 62, 76];

pub fn main() {
    let l: [[Fp; 11]; 5] = L.map(|row| row.map(|x| gf(x)));
    let r: [[Fp; 11]; 5] = R.map(|row| row.map(|y| gf(y)));
    let out: [[Fp; 11]; 5] = OUT.map(|row| row.map(|z| gf(z)));
    let witness = WITNESS.map(|x| gf(x));

    // L will be multiplied by G₁
    // R will be multiplied by G₂
    let l_mul = witness_mul(&l, &witness);
    let r_mul = witness_mul(&r, &witness);
    let out_mul = witness_mul(&out, &witness);

    // The hadamard product will contain a pairing of G₁ • G₂
    let product = hadamard_product(&l_mul, &r_mul);

    // G1 • (39, 54) — 𝔽157
    // G2 • (31 + 153α, 5 + 59α) — 𝔽157²

    print!("[");
    for value in product {
        print!("{}, ", value);
    }
    print!("]\n");

    assert_eq!(product, out_mul);
}

fn witness_mul(vector: &[[Fp; 11]; 5], witness: &[Fp; 11]) -> [Fp; 5] {
    let mut result: [Fp; 5] = [gf(0); 5];

    for i in 0..result.len() {
        let sum = &mut result[i];

        for j in 0..11 {
            *sum = *sum + (vector[i][j] * witness[j]);
        }
    }

    result
}

fn hadamard_product<T>(l_matrix: &[T; 5], r_matrix: &[T; 5]) -> [T; 5]
where
    T: ops::Mul<Output = T> + Copy,
{
    let result: [T; 5] = std::array::from_fn(|i| l_matrix[i] * r_matrix[i]);

    result
}

pub fn gf(value: i64) -> Fp {
    Fp::new(value, MODULUS).unwrap()
}
