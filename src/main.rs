pub mod maths;
pub mod problem;
pub mod snark;

use crate::snark::{qap, r1cs, srs::generate_srs};

fn main() {
    r1cs::main();
    let qap = qap::main();
    let (srs_g1, srs_g2, srs_hx) = generate_srs(
        qap.l_poly.len() as u32 - 1,
        qap.r_poly.len() as u32 - 1,
        &qap.t_poly,
    );
}
