use crate::{
    maths::lagrange::{
        Literal, lagrange_interpolate, poly_add, poly_divide, poly_mul, poly_scale, poly_subtract,
    },
    snark::{
        proof::{COLS, L, OUT, R, ROWS, SUBGROUP_ORDER, WITNESS},
        r1cs::gf,
    },
};
use cryptography::exercises::{ec_point::Field, finite_field::Fp};

type Polynomial<F: Field> = Vec<F>;

impl Literal for Fp {
    fn raw(&self) -> u32 {
        self.num
    }
}

pub struct Qap {
    pub l_poly: Vec<Fp>,
    pub r_poly: Vec<Fp>,
    pub o_poly: Vec<Fp>,
    pub t_poly: Vec<Fp>,
    pub h_poly: Vec<Fp>,
}

pub fn main() -> Qap {
    let polynomials = interpolate_r1cs_matrices();

    let u_poly = polynomials.u_poly();
    let v_poly = polynomials.v_poly();
    let w_poly = polynomials.w_poly();
    let t_poly = compute_vanishing_polynomial();

    let numerator = poly_subtract(&poly_mul(&u_poly, &v_poly), &w_poly);
    let (h_poly, remainder) = poly_divide(&numerator, &t_poly);

    assert!(remainder.iter().all(|&x| x.is_zero()));

    Qap {
        l_poly: u_poly,
        r_poly: v_poly,
        o_poly: w_poly,
        t_poly,
        h_poly,
    }
}

pub struct R1CSPolynomial<F: Field> {
    u: [Polynomial<F>; COLS],
    v: [Polynomial<F>; COLS],
    w: [Polynomial<F>; COLS],
}

impl<F: Field> R1CSPolynomial<F> {
    fn witness_weighted_sum(polys: &[Polynomial<F>; COLS]) -> Polynomial<F> {
        polys
            .iter()
            .enumerate()
            .map(|(idx, poly)| {
                let witness = F::one(SUBGROUP_ORDER).scalar_mul(WITNESS[idx] as u32);

                poly_scale(poly, witness)
            })
            .reduce(|acc, e| poly_add(&acc, &e))
            .unwrap()
    }

    pub fn get_u_poly_at(&self, col: usize) -> &Polynomial<F> {
        assert!(col < COLS, "u_poly: column out of bounds");

        &self.u[col]
    }

    pub fn get_v_poly_at(&self, col: usize) -> &Polynomial<F> {
        assert!(col < COLS, "v_poly: column out of bounds");

        &self.v[col]
    }

    pub fn get_w_poly_at(&self, col: usize) -> &Polynomial<F> {
        assert!(col < COLS, "w_poly: column out of bounds");

        &self.w[col]
    }

    pub fn u_poly(&self) -> Polynomial<F> {
        Self::witness_weighted_sum(&self.u)
    }

    pub fn v_poly(&self) -> Polynomial<F> {
        Self::witness_weighted_sum(&self.v)
    }

    pub fn w_poly(&self) -> Polynomial<F> {
        Self::witness_weighted_sum(&self.w)
    }
}

pub fn interpolate_r1cs_matrices() -> R1CSPolynomial<Fp> {
    let l_matrix: [[Fp; COLS]; ROWS] = L.map(|row| row.map(|x| gf(x)));
    let r_matrix: [[Fp; COLS]; ROWS] = R.map(|row| row.map(|y| gf(y)));
    let out_matrix: [[Fp; COLS]; ROWS] = OUT.map(|row| row.map(|z| gf(z)));

    let x_coords: [Fp; ROWS] = std::array::from_fn(|i| gf((i + 1) as i64));

    let mut u_poly: [Polynomial<Fp>; COLS] = std::array::from_fn(|_| Polynomial::new());
    let mut v_poly: [Polynomial<Fp>; COLS] = std::array::from_fn(|_| Polynomial::new());
    let mut w_poly: [Polynomial<Fp>; COLS] = std::array::from_fn(|_| Polynomial::new());

    for col in 0..COLS {
        let mut u: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];
        let mut v: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];
        let mut w: [Fp; ROWS] = [Fp::zero(SUBGROUP_ORDER); ROWS];

        for row in 0..ROWS {
            u[row] = l_matrix[row][col];
            v[row] = r_matrix[row][col];
            w[row] = out_matrix[row][col];
        }

        u_poly[col] = lagrange_interpolate(&x_coords, &u);
        v_poly[col] = lagrange_interpolate(&x_coords, &v);
        w_poly[col] = lagrange_interpolate(&x_coords, &w);
    }

    R1CSPolynomial {
        u: u_poly,
        v: v_poly,
        w: w_poly,
    }
}

pub fn compute_vanishing_polynomial() -> Polynomial<Fp> {
    let mut t_poly: Vec<Fp> = vec![gf(-1), gf(1)];

    for i in 1..ROWS {
        t_poly = poly_mul(&t_poly, &vec![gf(-1 * (i as i64 + 1)), gf(1)])
    }

    t_poly
}
