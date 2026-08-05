use std::ops::Mul;

use crate::snark::{key_utils::get_default_generators, witness::SUBGROUP_ORDER};
use cryptography::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::field_extension::Fp2,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnalogGT {
    exponent: Fp,
}

impl AnalogGT {
    pub fn identity() -> Self {
        Self::from_exponent(0)
    }

    pub fn generator() -> Self {
        Self::from_exponent(1)
    }

    pub fn from_exponent(exponent: u32) -> Self {
        Self {
            exponent: Fp::new(exponent as i64, SUBGROUP_ORDER).unwrap(),
        }
    }

    pub fn exponent(&self) -> Fp {
        self.exponent
    }

    pub fn pow(self, scalar: u32) -> Self {
        Self {
            exponent: self.exponent.scalar_mul(scalar),
        }
    }
}

impl Mul for AnalogGT {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            exponent: self.exponent + rhs.exponent,
        }
    }
}

pub fn analog_pairing(g1_point: Point<Fp>, g2_point: Point<Fp2>) -> AnalogGT {
    let (g1, g2) = get_default_generators();

    let g1_scalar = discrete_log(g1, g1_point).expect("G1 point is not in the toy subgroup");
    let g2_scalar = discrete_log(g2, g2_point).expect("G2 point is not in the toy subgroup");

    AnalogGT::generator().pow(g1_scalar * g2_scalar)
}

fn discrete_log<F>(generator: Point<F>, point: Point<F>) -> Option<u32>
where
    F: Field,
{
    for scalar in 0..SUBGROUP_ORDER {
        if generator.scalar_mul(scalar) == point {
            return Some(scalar);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toy_pairing_maps_generators_to_gt_generator() {
        let (g1, g2) = get_default_generators();

        assert_eq!(analog_pairing(g1, g2), AnalogGT::generator());
    }

    #[test]
    fn toy_pairing_is_bilinear_over_scalar_multiplication() {
        let (g1, g2) = get_default_generators();
        let a = 7;
        let b = 11;

        let lhs = analog_pairing(g1.scalar_mul(a), g2.scalar_mul(b));
        let rhs = analog_pairing(g1, g2).pow(a * b);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn toy_pairing_product_adds_target_group_exponents() {
        let (g1, g2) = get_default_generators();
        let a = 7;
        let b = 11;

        let g1_sum = (g1.scalar_mul(a) + g1.scalar_mul(b)).unwrap();
        let lhs = analog_pairing(g1_sum, g2);
        let rhs = analog_pairing(g1.scalar_mul(a), g2) * analog_pairing(g1.scalar_mul(b), g2);

        assert_eq!(lhs, rhs);
    }
}
