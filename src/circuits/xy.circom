pragma circom 2.0.0;

template XY() {
    signal input x;
    signal input y;
    signal output z;

    z <== x * y;

    // SOLUTION
    // witness vector = [1, z, x, y]
    // Output         = [0, 1, 0, 0]
    // Left           = [0, 0, 1, 0]
    // Right          = [0, 0, 0, 1]
}

component main = XY();
