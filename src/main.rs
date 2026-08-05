pub mod maths;
pub mod problem;
pub mod snark;

use cryptography::exercises::finite_field::Fp;

use crate::{
    maths::lagrange::print_polynomial,
    snark::{
        key,
        key_utils::fhe_polynomial_eval,
        qap::{self, Qap},
        r1cs::{self, gf},
        verify::verify,
        witness::{PUBLIC_INPUTS, WITNESS},
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
    let a = proving_key.alpha1 + fhe_polynomial_eval(&u_poly, &proving_key.a_srs);
    let b = proving_key.beta2 + fhe_polynomial_eval(&v_poly, &proving_key.b_srs);

    let witness = WITNESS.into_iter().map(|x| gf(x)).collect::<Vec<Fp>>();
    let c = fhe_polynomial_eval(
        &witness[PUBLIC_INPUTS..].to_vec(),
        &proving_key.private_phi1,
    ) + fhe_polynomial_eval(&h_poly, &proving_key.quotient_srs);

    // VERIFICATION
    // -------------------------------------
    let check = verify(
        a.unwrap(),
        b.unwrap(),
        c.unwrap(),
        &witness[0..PUBLIC_INPUTS].to_vec(),
        verification_key,
    );
    println!("Groth16 verification => {}", check);

    // TODO:
    // -------------------------------------
    // 1. encrypt proof using `r` and `s`
}
