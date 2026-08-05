use cryptography::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::field_extension::Fp2,
};

use crate::snark::{key_utils::get_default_generators, pairing::analog_pairing};

pub struct VerificationKey<F: Field, Ex: Field> {
    // [α]₁
    pub alpha1: Point<F>,

    // [β]₂
    pub beta2: Point<Ex>,
}

pub fn verify(a: Point<Fp>, b: Point<Fp2>, c: Point<Fp>, key: VerificationKey<Fp, Fp2>) -> bool {
    #[allow(non_snake_case)]
    let (_, G2) = get_default_generators();
    let lhs = analog_pairing(a, b);
    let rhs = analog_pairing(key.alpha1, key.beta2) * analog_pairing(c, G2);

    lhs == rhs
}
