use std::ops::Mul;

use crate::{
    maths::lagrange::{Literal, poly_eval},
    snark::{
        key_utils::{compute_srs, get_default_generators},
        qap::{compute_vanishing_polynomial, interpolate_r1cs_matrices},
        r1cs::gf as gf_subgroup,
        witness::{COLS, ROWS, SUBGROUP_ORDER},
    },
};
use cryptography::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::field_extension::Fp2,
};
use rand::{self, RngExt};

struct ToxicWaste {
    alpha: Fp,
    beta: Fp,
    gamma: Fp,
    delta: Fp,
    tau: Fp,
}

pub struct ProvingKey<F: Field, Ex: Field> {
    // [α]₁
    pub alpha1: Point<F>,

    // [β]₂
    pub beta2: Point<Ex>,

    // [γ]₂
    pub gamma2: Point<Ex>,

    // [δ]₂
    pub delta2: Point<Ex>,

    // [Φᵢ ... ]
    pub phi1: [Point<F>; COLS],

    // [τⁱ ... ]₁
    pub a_srs: Vec<Point<F>>,

    // [τⁱ ... ]₂
    pub b_srs: Vec<Point<Ex>>,

    // T(x)•τⁱ — used by prover to compute H(x)
    pub quotient_srs: Vec<Point<F>>,
}

fn generate_toxic_waste() -> ToxicWaste {
    let mut rng = rand::rng();

    let alpha = rng.random_range(1..SUBGROUP_ORDER);
    let beta = rng.random_range(1..SUBGROUP_ORDER);
    let gamma = rng.random_range(1..SUBGROUP_ORDER);
    let delta = rng.random_range(1..SUBGROUP_ORDER);
    let tau = rng.random_range(1..SUBGROUP_ORDER);

    ToxicWaste {
        alpha: gf_subgroup(alpha as i64),
        beta: gf_subgroup(beta as i64),
        gamma: gf_subgroup(gamma as i64),
        delta: gf_subgroup(delta as i64),
        tau: gf_subgroup(tau as i64),
    }
}

pub fn compute() -> ProvingKey<Fp, Fp2> {
    #[allow(non_snake_case)]
    let (G1, G2) = get_default_generators();

    let ToxicWaste {
        alpha,
        beta,
        gamma,
        delta,
        tau,
    } = generate_toxic_waste();

    let polynomials = interpolate_r1cs_matrices();
    let mut phi: [Fp; COLS] = std::array::from_fn(|_| gf_subgroup(0));

    for col in 0..COLS {
        let u_i = poly_eval(polynomials.get_u_poly_at(col), tau);
        let v_i = poly_eval(polynomials.get_v_poly_at(col), tau);
        let w_i = poly_eval(polynomials.get_w_poly_at(col), tau);

        phi[col] = beta.mul(u_i) + alpha.mul(v_i) + w_i;
    }

    let vanishing_poly = compute_vanishing_polynomial();
    let vanishing_poly_eval = poly_eval(&vanishing_poly, tau);

    ProvingKey {
        alpha1: G1.scalar_mul(alpha.raw()),
        beta2: G2.scalar_mul(beta.raw()),
        gamma2: G2.scalar_mul(gamma.raw()),
        delta2: G2.scalar_mul(delta.raw()),
        phi1: phi.map(|x| G1.scalar_mul(x.raw())),
        a_srs: compute_srs(tau, G1, ROWS - 1),
        b_srs: compute_srs(tau, G2, ROWS - 1),
        quotient_srs: compute_srs(tau, G1.scalar_mul(vanishing_poly_eval.raw()), ROWS - 2),
    }
}
