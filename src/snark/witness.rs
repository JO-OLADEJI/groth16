use cryptography::exercises::ec_point::Field;

pub static SUBGROUP_ORDER: u32 = 79;
pub static EC_POINT_MODULUS: u32 = 157;

pub struct EcCurve<T: Field> {
    pub a: T,
    pub b: T,
}

pub static ROWS: usize = 5;
pub static COLS: usize = 11;

pub static L: [[i64; COLS]; ROWS] = [
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
];

pub static R: [[i64; COLS]; ROWS] = [
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
];

pub static OUT: [[i64; COLS]; ROWS] = [
    [0, 0, 0, 0, -3, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, -3, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, -3, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -3],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
];

pub static WITNESS: [i64; COLS] = [1, 62, 76, 47, 78, 73, 72, 49, 15, 76, 44];
