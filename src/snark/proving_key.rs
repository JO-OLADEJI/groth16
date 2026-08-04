use std::ops::Mul;

use crate::{
    maths::lagrange::{Literal, poly_eval},
    snark::{
        proof::{COLS, ROWS, SUBGROUP_ORDER},
        qap::{compute_vanishing_polynomial, interpolate_r1cs_matrices},
        r1cs::gf as gf_subgroup,
        srs::{compute_srs, get_default_generators},
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

struct Groth16Scalars {
    alpha: Fp,
    beta: Fp,
    gamma: Fp,
    delta: Fp,
    tau: Fp,
}

pub struct ProvingKey<F: Field, Ex: Field> {
    alpha1: Point<F>,
    beta2: Point<Ex>,
    phi1: [Point<F>; COLS],
    a_srs: Vec<Point<F>>,
    b_srs: Vec<Point<Ex>>,
    quotient_srs: Vec<Point<F>>,
}

fn generate_groth16_scalars() -> Groth16Scalars {
    let mut rng = rand::rng();

    let alpha = rng.random_range(1..SUBGROUP_ORDER);
    let beta = rng.random_range(1..SUBGROUP_ORDER);
    let gamma = rng.random_range(1..SUBGROUP_ORDER);
    let delta = rng.random_range(1..SUBGROUP_ORDER);
    let tau = rng.random_range(1..SUBGROUP_ORDER);

    Groth16Scalars {
        alpha: gf_subgroup(alpha as i64),
        beta: gf_subgroup(beta as i64),
        gamma: gf_subgroup(gamma as i64),
        delta: gf_subgroup(delta as i64),
        tau: gf_subgroup(tau as i64),
    }
}

// 1. generate 5 random field elements in subgroup field
// [α, β, γ, δ, τ]
// 2. Compute the evaluation of -> β•u(x)ᵢ + α•v(x)ᵢ + w(x)ᵢ
// 3. Compute the evaluation of T(x)
pub fn compute() -> ProvingKey<Fp, Fp2> {
    #[allow(non_snake_case)]
    let (G1, G2) = get_default_generators();
    let Groth16Scalars {
        alpha,
        beta,
        gamma,
        delta,
        tau,
    } = generate_groth16_scalars();

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
        // [α]₁
        alpha1: G1.scalar_mul(alpha.raw()),

        // [β]₂
        beta2: G2.scalar_mul(beta.raw()),

        // [Φᵢ ... ]
        phi1: phi.map(|x| G1.scalar_mul(x.raw())),

        // [τ² ... ]₁
        a_srs: compute_srs(tau, G1, ROWS - 1),

        // [τ² ... ]₂
        b_srs: compute_srs(tau, G2, ROWS - 1),

        // T(x)•τⁱ — used by prover to compute H(x)
        quotient_srs: compute_srs(tau, G1.scalar_mul(vanishing_poly_eval.raw()), ROWS - 2),
    }
}
