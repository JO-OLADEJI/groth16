use cryptography::{
    exercises::{
        ec_point::{Field, Point},
        finite_field::Fp,
    },
    extension_fields::field_extension::Fp2,
};

use crate::{
    maths::lagrange::Literal,
    snark::witness::{EC_POINT_MODULUS, EcCurve},
};

pub fn gf_higher_order(value: i64) -> Fp {
    Fp::new(value, EC_POINT_MODULUS).unwrap()
}

fn gf_extension(a: Fp, b: Fp) -> Fp2 {
    Fp2 { a, b }
}

pub fn get_default_generators() -> (Point<Fp>, Point<Fp2>) {
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

pub fn compute_srs<F: Field>(tau: Fp, point: Point<F>, poly_degree: usize) -> Vec<Point<F>> {
    let mut result = vec![];

    // degree `n` => `n + 1` terms (max)
    for i in 0..=poly_degree {
        let t = tau.pow(i as u32);

        result.push(point.scalar_mul(t.raw()));
    }

    result
}

pub fn fhe_polynomial_eval<F: Field>(poly: &[Fp], srs: &[Point<F>]) -> Point<F> {
    assert_eq!(poly.len(), srs.len(), "key mismatch for poly eval");

    srs.iter()
        .enumerate()
        .map(|(idx, point)| point.scalar_mul(poly[idx].raw()))
        .reduce(|acc, e| (acc + e).unwrap())
        .unwrap()
}
