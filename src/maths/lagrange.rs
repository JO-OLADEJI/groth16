// NOTE: totally untested code
use std::ops::{Add, Div, Mul, Sub};

pub trait Field:
    Copy
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

/// Multiply two polynomials.
/// Coefficients are in ascending order:
/// [a0, a1, a2] = a0 + a1*x + a2*x²
fn poly_mul<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let mut out = vec![F::zero(); a.len() + b.len() - 1];

    for i in 0..a.len() {
        for j in 0..b.len() {
            out[i + j] = out[i + j] + a[i] * b[j];
        }
    }

    out
}

/// Add two polynomials.
fn poly_add<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut out = vec![F::zero(); n];

    for i in 0..n {
        let ai = if i < a.len() { a[i] } else { F::zero() };
        let bi = if i < b.len() { b[i] } else { F::zero() };
        out[i] = ai + bi;
    }

    out
}

/// Scale a polynomial.
fn poly_scale<F: Field>(poly: &[F], s: F) -> Vec<F> {
    poly.iter().map(|&c| c * s).collect()
}

/// Compute the interpolating polynomial.
///
/// xs = x coordinates
/// ys = y coordinates
///
/// Returns coefficients in ascending order.
pub fn lagrange_interpolate<F: Field>(xs: &[F], ys: &[F]) -> Vec<F> {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();

    let mut result = vec![F::zero(); n];

    for i in 0..n {
        // Numerator polynomial
        let mut basis = vec![F::one()];

        // Denominator scalar
        let mut denom = F::one();

        for j in 0..n {
            if i == j {
                continue;
            }

            // Multiply by (x - x_j)
            basis = poly_mul(&basis, &[F::zero() - xs[j], F::one()]);

            denom = denom * (xs[i] - xs[j]);
        }

        let basis = poly_scale(&basis, ys[i] / denom);

        result = poly_add(&result, &basis);
    }

    result
}
