use crate::snark::proof::SUBGROUP_ORDER;
use cryptography::exercises::ec_point::Field;

pub trait Literal {
    fn raw(&self) -> u32;
}

/// Multiply two polynomials.
/// Coefficients are in ascending order:
/// [a0, a1, a2] = a0 + a1*x + a2*x²
pub fn poly_mul<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let mut out = vec![F::zero(a[0].modulus()); a.len() + b.len() - 1];

    for i in 0..a.len() {
        for j in 0..b.len() {
            out[i + j] = out[i + j] + a[i] * b[j];
        }
    }

    out
}

/// Add two polynomials.
pub fn poly_add<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut out = vec![F::zero(SUBGROUP_ORDER); n];

    for i in 0..n {
        let ai = if i < a.len() {
            a[i]
        } else {
            F::zero(SUBGROUP_ORDER)
        };
        let bi = if i < b.len() {
            b[i]
        } else {
            F::zero(SUBGROUP_ORDER)
        };
        out[i] = ai + bi;
    }

    out
}

/// Subtract two polynomials.
pub fn poly_subtract<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len().max(b.len());
    let mut out = vec![F::zero(SUBGROUP_ORDER); n];

    for i in 0..n {
        let ai = if i < a.len() {
            a[i]
        } else {
            F::zero(SUBGROUP_ORDER)
        };
        let bi = if i < b.len() {
            b[i]
        } else {
            F::zero(SUBGROUP_ORDER)
        };
        out[i] = ai - bi;
    }

    out
}

/// Scale a polynomial.
pub fn poly_scale<F: Field>(poly: &[F], s: F) -> Vec<F> {
    poly.iter().map(|&c| c * s).collect()
}

fn poly_degree<F: Field>(poly: &[F]) -> Option<usize> {
    poly.iter().rposition(|&c| !c.is_zero())
}

fn trim_poly<F: Field>(poly: &mut Vec<F>) {
    while poly.len() > 1 && poly.last().is_some_and(|&c| c.is_zero()) {
        poly.pop();
    }

    if poly.is_empty() {
        poly.push(F::zero(SUBGROUP_ORDER));
    }
}

/// Divide one polynomial by another.
///
/// Coefficients are in ascending order:
/// [a0, a1, a2] = a0 + a1*x + a2*x²
///
/// Returns (quotient, remainder).
pub fn poly_divide<F: Field>(dividend: &[F], divisor: &[F]) -> (Vec<F>, Vec<F>) {
    assert!(!divisor.is_empty(), "cannot divide by an empty polynomial");

    let divisor_degree = poly_degree(divisor).expect("cannot divide by a zero polynomial");

    let Some(dividend_degree) = poly_degree(dividend) else {
        return (vec![F::zero(SUBGROUP_ORDER)], vec![F::zero(SUBGROUP_ORDER)]);
    };

    if dividend_degree < divisor_degree {
        let mut remainder = dividend.to_vec();
        trim_poly(&mut remainder);
        return (vec![F::zero(SUBGROUP_ORDER)], remainder);
    }

    let mut quotient = vec![F::zero(SUBGROUP_ORDER); dividend_degree - divisor_degree + 1];
    let mut remainder = dividend.to_vec();
    trim_poly(&mut remainder);

    let divisor_lead = divisor[divisor_degree];

    while let Some(remainder_degree) = poly_degree(&remainder) {
        if remainder_degree < divisor_degree {
            break;
        }

        let degree_diff = remainder_degree - divisor_degree;
        let coeff = remainder[remainder_degree] / divisor_lead;
        quotient[degree_diff] = coeff;

        for i in 0..=divisor_degree {
            let remainder_index = degree_diff + i;
            remainder[remainder_index] = remainder[remainder_index] - coeff * divisor[i];
        }

        trim_poly(&mut remainder);
    }

    trim_poly(&mut quotient);

    (quotient, remainder)
}

/// Evaluate a polynomial
pub fn poly_eval<F: Field>(poly: &[F], x: F) -> F {
    let mut result = F::zero(SUBGROUP_ORDER);

    for (exp, &value) in poly.iter().enumerate().rev() {
        if !value.is_zero() {
            result = result + value * x.pow(exp as u32);
        }
    }

    result
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

    let mut result = vec![F::zero(SUBGROUP_ORDER); n];

    for i in 0..n {
        // Numerator polynomial
        let mut basis = vec![F::one(SUBGROUP_ORDER)];

        // Denominator scalar
        let mut denom = F::one(SUBGROUP_ORDER);

        for j in 0..n {
            if i == j {
                continue;
            }

            // Multiply by (x - x_j)
            basis = poly_mul(
                &basis,
                &[F::zero(SUBGROUP_ORDER) - xs[j], F::one(SUBGROUP_ORDER)],
            );

            denom = denom * (xs[i] - xs[j]);
        }

        let basis = poly_scale(&basis, ys[i] / denom);

        result = poly_add(&result, &basis);
    }

    result
}

pub fn print_polynomial<F>(coeffs: &[F], identifier: &str)
where
    F: Field + Literal,
{
    let mut terms = Vec::new();

    for (degree, &coeff) in coeffs.iter().enumerate().rev() {
        if coeff.is_zero() {
            continue;
        }

        let term = match degree {
            0 => format!("{}", coeff.raw()),
            1 => {
                if coeff == F::one(SUBGROUP_ORDER) {
                    "x".to_string()
                } else {
                    format!("{}x", coeff.raw())
                }
            }
            _ => {
                if coeff == F::one(SUBGROUP_ORDER) {
                    format!("x{}", to_superscript(degree))
                } else {
                    format!("{}x{}", coeff.raw(), to_superscript(degree))
                }
            }
        };

        terms.push(term);
    }

    let poly = if terms.is_empty() {
        String::from("0")
    } else {
        terms.join(" + ")
    };

    println!("{}", format!("{} => {}", identifier, poly));
}

fn to_superscript(s: usize) -> String {
    s.to_string()
        .chars()
        .map(|c| match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '-' => '⁻',
            '+' => '⁺',
            _ => c,
        })
        .collect()
}
