use crate::{
    maths::lagrange::{lagrange_interpolate, poly_add, poly_scale},
    snark::proof::{COLS, L, MODULUS, OUT, R, ROWS, WITNESS},
    snark::r1cs::gf,
};
use cryptography::exercises::{ec_point::Field, finite_field::Fp};

pub fn main() {
    let l_matrix: [[Fp; COLS]; ROWS] = L.map(|row| row.map(|x| gf(x)));
    let r_matrix: [[Fp; COLS]; ROWS] = R.map(|row| row.map(|y| gf(y)));
    let out_matrix: [[Fp; COLS]; ROWS] = OUT.map(|row| row.map(|z| gf(z)));
    let witness: [Fp; COLS] = WITNESS.map(|x| gf(x));

    let x_coords: [Fp; ROWS] = std::array::from_fn(|i| gf((i + 1) as i64));

    let mut u_poly: Vec<Fp> = vec![];
    let mut v_poly: Vec<Fp> = vec![];
    let mut w_poly: Vec<Fp> = vec![];

    for col in 0..COLS {
        let mut u: [Fp; ROWS] = [Fp::zero(MODULUS); ROWS];
        let mut v: [Fp; ROWS] = [Fp::zero(MODULUS); ROWS];
        let mut w: [Fp; ROWS] = [Fp::zero(MODULUS); ROWS];

        for row in 0..ROWS {
            u[row] = l_matrix[row][col];
            v[row] = r_matrix[row][col];
            w[row] = out_matrix[row][col];
        }

        u_poly = poly_add(
            &u_poly,
            &poly_scale(&lagrange_interpolate(&x_coords, &u), witness[col]),
        );
        v_poly = poly_add(
            &v_poly,
            &poly_scale(&lagrange_interpolate(&x_coords, &v), witness[col]),
        );
        w_poly = poly_add(
            &w_poly,
            &poly_scale(&lagrange_interpolate(&x_coords, &w), witness[col]),
        );
    }

    // NOTE: evaluating `u_poly • v_poly =? w_poly` at a random point `τ` does not guarantee a balance
    // because vectors and the polynomial that interpolates them have homomorphic claims only on addition
    // and not hadamard product / multiplication.
    // See: https://rareskills.io/post/quadratic-arithmetic-program#polynomial-degree-imbalance

    // NEXT: compute t(x) and h(x)

    // LAST: evaluate all on a random `τ`
    let _tau = gf(23);
}
