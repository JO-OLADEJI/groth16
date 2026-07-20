use cryptography::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::field_extension::Fp2,
};
use rand::RngExt;

use crate::{
    maths::lagrange::poly_eval,
    snark::{
        proof::{EC_POINT_MODULUS, EcCurve, SUBGROUP_ORDER},
        r1cs::gf as gf_subgroup,
    },
};

pub fn generate_srs(
    l_poly_degree: u32,
    r_poly_degree: u32,
    t_poly: &[Fp],
) -> (Vec<Point<Fp>>, Vec<Point<Fp2>>, Vec<Point<Fp>>) {
    let x = rand::rng().random_range(1..SUBGROUP_ORDER);
    let tau = gf_subgroup(x as i64); // τ
    println!("Debug: tau(τ) = {}", tau);

    #[allow(non_snake_case)]
    let (G1, G2) = get_default_generators();

    // given QAP: `L(x) • R(x) − Out(x) = h(x) • t(x)`
    // deg(L), deg(R), deg(Out) = n - 1
    // deg(t) = n
    // deg(h) = n - 2
    // ...
    // where n = number of constraints
    let mut omega = vec![]; // Ω₀ => τ⁰(G1) ...
    let mut theta = vec![]; // θ₀ => τ⁰(G2) ...
    let mut gamma = vec![]; // γ₀ => t(τ)τ⁰(G2)

    let r1cs_poly_max_terms = l_poly_degree.max(r_poly_degree) + 1;
    let hx_max_terms = t_poly.len() - 2; // polynomial's length represents the number of terms => deg + 1

    for i in 0..r1cs_poly_max_terms {
        let scalar = tau.pow(i);
        omega.push(G1.scalar_mul(scalar.num));
        theta.push(G2.scalar_mul(scalar.num));
    }

    let tx_eval_tau = poly_eval(t_poly, tau);
    for i in 0..hx_max_terms {
        let scalar = tau.pow(i as u32) * tx_eval_tau;
        gamma.push(G1.scalar_mul(scalar.num));
    }

    (omega, theta, gamma)
}

fn gf_higher_order(value: i64) -> Fp {
    Fp::new(value, EC_POINT_MODULUS).unwrap()
}

fn gf_extension(a: Fp, b: Fp) -> Fp2 {
    Fp2 { a, b }
}

fn get_default_generators() -> (Point<Fp>, Point<Fp2>) {
    // For PRIME_ORDER = 157
    // Curve: y² = x³ - 1x - 1
    // ----------------------------------------------------------
    // No. of points #E(Fp): 158
    // Potential subgroups order `r` for G1: [2, 79]
    // embedding degrees: {2: Some(1), 79: Some(2)}
    // MODULUS = subgroup order `r` -> 79

    // G1 • (39, 54) — 𝔽157
    // G2 • (31 + 153α, 5 + 59α) — 𝔽157²

    let zero = gf_higher_order(0);
    let curve = EcCurve {
        a: gf_higher_order(-1),
        b: gf_higher_order(-1),
    };

    let x1 = gf_higher_order(39);
    let y1 = gf_higher_order(54);
    let linear_generator = Point::new(curve.a, curve.b, Some(x1), Some(y1)).unwrap();

    let x2a = gf_higher_order(31);
    let x2b = gf_higher_order(153);
    let y2a = gf_higher_order(5);
    let y2b = gf_higher_order(59);
    let extension_generator = Point::new(
        gf_extension(curve.a, zero),
        gf_extension(curve.b, zero),
        Some(gf_extension(x2a, x2b)),
        Some(gf_extension(y2a, y2b)),
    )
    .unwrap();

    (linear_generator, extension_generator)
}
