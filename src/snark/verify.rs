use cryptography::{
    exercises::{ec_point::Point, finite_field::Fp},
    extension_fields::field_extension::Fp2,
};

use crate::snark::{key::VerificationKey, key_utils::fhe_polynomial_eval, pairing::analog_pairing};

pub fn verify(
    a: Point<Fp>,
    b: Point<Fp2>,
    c: Point<Fp>,
    public_witness: &[Fp],
    key: VerificationKey<Fp, Fp2>,
) -> bool {
    let public_witness_eval = fhe_polynomial_eval(&public_witness, &key.public_phi1);

    let lhs = analog_pairing(a, b);
    let rhs = analog_pairing(key.alpha1, key.beta2)
        * analog_pairing(public_witness_eval, key.gamma2)
        * analog_pairing(c, key.delta2);

    lhs == rhs
}
