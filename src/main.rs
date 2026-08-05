pub mod maths;
pub mod problem;
pub mod snark;

use cryptography::exercises::finite_field::Fp;

use crate::{
    maths::lagrange::print_polynomial,
    snark::{
        key_utils::fhe_polynomial_eval,
        proving_key,
        qap::{self, Qap},
        r1cs::{self, gf},
        verify::{VerificationKey, verify},
        witness::WITNESS,
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

    let key = proving_key::compute();

    // PROOF GENERATION
    // -------------------------------------
    let a = key.alpha1 + fhe_polynomial_eval(&u_poly, &key.a_srs);
    let b = key.beta2 + fhe_polynomial_eval(&v_poly, &key.b_srs);

    let witness = WITNESS.into_iter().map(|x| gf(x)).collect::<Vec<Fp>>();
    let c = fhe_polynomial_eval(&witness, &key.phi1.to_vec())
        + fhe_polynomial_eval(&h_poly, &key.quotient_srs);

    // VERIFICATION
    // -------------------------------------
    let check = verify(
        a.unwrap(),
        b.unwrap(),
        c.unwrap(),
        VerificationKey {
            alpha1: key.alpha1,
            beta2: key.beta2,
        },
    );
    println!("is proof correct: {}", check);

    // TODO:
    // -------------------------------------
    // 1. test prover equation, splitting public & private inputs
    // 2. encrypt proof using `r` and `s`
    // 3. generate the proving & verification key in a JSON file
}
