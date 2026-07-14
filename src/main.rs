pub mod maths;
pub mod problem;
pub mod snark;

use crate::snark::{qap, r1cs};

fn main() {
    r1cs::main();
    qap::main();
}
