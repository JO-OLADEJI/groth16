pragma circom 2.1.6;

template MerkleInclusion() {
    /*
     * 0: nullifier
     * 1: secret
     * 2: L₁ hash
     * 3: d-bit
     * 4: L₂ hash
     * 5: d-bit
     * 6: L₃ hash
     * 7: d-bit
     */
    signal input in[8];
    signal input merkleRoot;
    signal input nullifierHash;

    component hashes[4];
    signal lFlags[4];
    signal rFlags[4];

    hashes[0] = Hash();
    hashes[0].l <== in[0];
    hashes[0].r <== in[1];
    lFlags[0] <-- 0;
    rFlags[0] <-- 0;

    var idx = 1;

    for (var i = 3; i <= 7; i += 2) {
        // hash direction-bit binary constraints — {0, 1}
        in[i] * (in[i] - 1) === 0;

        /*
         * Rationale:
         * For the hash to be computed correctly, the arrangements of arguments into the hash function matter
         * the `d-bit(s)` in the input signal is a binary flag that specifies if the hashes should be re-arranged:
         * 0 = leave order as is... supplied hash goes in 2nd
         * 1 = re-arrange order ... supplied hash goes in 1st
         * the `lFlags` and `rFlags` array exist as the first part of a Quin selector (sort of)
         * signal left <-- (1 - in[i]) * hashes[idx].out + in[i] * in[i-1];
         * signal right <-- (1 - in[i]) * in[i-1] + in[i] * hashes[idx].out;
         * where:
         * i ∈ {3, 5, 7}
         * `in[i] * in[i-1]` is zeroed out if `in[i]` d-bit is `0` and the <left> signal is set to `(1 - in[i]) * hash0.out` which is the circom-computed hash
         * `in[i] * hash0.out` is zeroed out if `in[i]` d-bit is `0` and the <right> signal is set to `(1 - in[i]) * in[i-1]` which is the supplied hash
         * and setting the `in[i]` d-bit to `1` causes both signals to switch values
         * and `hashes[]` keeps track of the circom-computed hash function outputs
         */
        hashes[idx] = Hash();
        lFlags[idx] <== (1 - in[i]) * hashes[idx - 1].out;
        rFlags[idx] <== (1 - in[i]) * in[i - 1];

        hashes[idx].l <== lFlags[idx] + in[i] * in[i - 1];
        hashes[idx].r <== rFlags[idx] + in[i] * hashes[idx - 1].out;

        idx++;
    }

    // merkle root constraint
    hashes[3].out === merkleRoot;

    // nullifier hash constraint
    signal nHash <== Hash()(in[0], 0);
    nHash === nullifierHash;
}

template Hash() {
    signal input l;
    signal input r;
    signal output out;

    var cHash = (l * l) + (3 * r);
    out <== cHash;
}

component main {public [merkleRoot, nullifierHash]} = MerkleInclusion();
