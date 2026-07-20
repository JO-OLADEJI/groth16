use crate::{
    maths::lagrange::{
        Literal, lagrange_interpolate, poly_add, poly_divide, poly_mul, poly_scale, poly_subtract,
        print_polynomial,
    },
    snark::{
        proof::{COLS, L, OUT, R, ROWS, SUBGROUP_ORDER, WITNESS},
        r1cs::gf,
        srs::generate_srs,
    },
};
use cryptography::exercises::{ec_point::Field, finite_field::Fp};

impl Literal for Fp {
    fn raw(&self) -> u32 {
        self.num
    }
}

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
        let mut u: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];
        let mut v: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];
        let mut w: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];

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

    // compute all polynomials in `u(x) • v(x) − w(x) = h(x) • t(x)`
    let mut t_poly: Vec<Fp> = vec![gf(-1), gf(1)];
    for i in 1..ROWS {
        t_poly = poly_mul(&t_poly, &vec![gf(-1 * (i as i64 + 1)), gf(1)])
    }

    let numerator = poly_subtract(&poly_mul(&u_poly, &v_poly), &w_poly);
    let (h_poly, remainder) = poly_divide(&numerator, &t_poly);

    assert!(remainder.iter().all(|&x| x.is_zero()));

    println!("\nPolynomials");
    println!("--------------------------------------------------------");
    print_polynomial(&u_poly, "u(x)");
    print_polynomial(&v_poly, "v(x)");
    print_polynomial(&w_poly, "w(x)");
    print_polynomial(&t_poly, "t(x)");
    print_polynomial(&h_poly, "h(x)");

    let (srs_g1, srs_g2, srs_hx) =
        generate_srs(u_poly.len() as u32 - 1, v_poly.len() as u32 - 1, &t_poly);

    println!("\n");
    println!("{:?}", srs_g1);
    println!("{:?}", srs_g2);
    println!("{:?}", srs_hx);
}
