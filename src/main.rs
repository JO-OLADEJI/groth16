pub mod maths;
pub mod problem;
pub mod snark;

use std::ops::Mul;

use cryptography::exercises::finite_field::Fp;
use rand::RngExt;

use crate::{
    maths::lagrange::{Literal, print_polynomial},
    snark::{
        key,
        key_utils::fhe_polynomial_eval,
        qap::{self, Qap},
        r1cs::{self, gf},
        verify::verify,
        witness::{PUBLIC_INPUTS, SUBGROUP_ORDER, WITNESS},
    },
};

fn main() {
    r1cs::validate();

    let Qap {
        u_poly,
        v_poly,
        w_poly,
        h_poly,
        t_poly,
    } = qap::main();

    print_polynomial(&u_poly, "u(x)");
    print_polynomial(&v_poly, "v(x)");
    print_polynomial(&w_poly, "w(x)");
    print_polynomial(&h_poly, "h(x)");
    print_polynomial(&t_poly, "t(x)");

    let (proving_key, verification_key) = key::compute();

    // PROOF GENERATION
    // -------------------------------------
    let mut rng = rand::rng();
    let r = gf(rng.random_range(1..SUBGROUP_ORDER) as i64);
    let s = gf(rng.random_range(1..SUBGROUP_ORDER) as i64);
    let witness = WITNESS.into_iter().map(|x| gf(x)).collect::<Vec<Fp>>();

    let a = proving_key.alpha1 + fhe_polynomial_eval(&u_poly, &proving_key.g1_srs);

    let b1 = proving_key.beta1 + fhe_polynomial_eval(&v_poly, &proving_key.g1_srs);
    let b2 = proving_key.beta2 + fhe_polynomial_eval(&v_poly, &proving_key.g2_srs);

    let c = fhe_polynomial_eval(
        &witness[PUBLIC_INPUTS..].to_vec(),
        &proving_key.private_phi1,
    ) + fhe_polynomial_eval(&h_poly, &proving_key.quotient_srs);
    let cx = c.unwrap() + a.clone().unwrap().scalar_mul(s.raw());
    let cy = cx.unwrap() + b1.unwrap().scalar_mul(r.raw());
    let cz = cy.unwrap() + (proving_key.delta1.scalar_mul(r.mul(s).raw()));

    #[allow(non_snake_case)]
    let A = a.unwrap() + proving_key.delta1.scalar_mul(r.raw());

    #[allow(non_snake_case)]
    let B = b2.unwrap() + proving_key.delta2.scalar_mul(s.raw());

    #[allow(non_snake_case)]
    let C = cz;

    // VERIFICATION
    // -------------------------------------
    let check = verify(
        A.unwrap(),
        B.unwrap(),
        C.unwrap(),
        &witness[0..PUBLIC_INPUTS].to_vec(),
        verification_key,
    );
    println!("Groth16 Verification => {}", check);
}
