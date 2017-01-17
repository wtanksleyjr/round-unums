/* Inspired^1 by:
 * https://groups.google.com/d/msg/unum-computing/UUpEd5fc3ro/QvJDImv8HwAJ
 * (from John L. Gustafson)
 * I am working on tools to populate the arithmetic tables automatically now. Ultimately I would
 * like to release a complete environment as I did for unums 1.0, with conversions from real
 * numbers to unums/SORNs and back, a full set of elementary functions, fused operations, a
 * lossless polynomial evaluator, and so on.
 * Notice that for multiplication and division, the projective real line becomes something very
 * like a circular slide rule! That is, the lattice of numbers is so close to perfect logarithmic
 * spacing that you can multiply numbers by summing their rotations about the circle. My plan is
 * to use that as a hash function, guessing where the product or quotient is, the test against the
 * predicted value and move up or down until the correct unum is determined. That should only take
 * O(1) time, so populating an N by N table should take order N^2 time, tolerable for low N.
 * For larger N, I envision an algorithm that only uses table look-up as the innermost kernel of
 * a procedure that looks more like that used for decimal floats: determine the power of ten of
 * the inputs and compute the power of ten of the result, then use tables within the decade.
 * That should make 32-bit accuracy tractable (64-bit SORN for contiguous sets).
 * The idea is so simple, I wouldn't be surprised if C and C++ versions pop up long before I get
 * my prototype done! Please keep me informed of progress.
 * - John G.
 *
 * Inspired^2 by:
 * https://groups.google.com/d/msg/unum-computing/CAKFOztPkqA/v41j0yxkAgAJ
 * P.S. Here are the decimals for a type 2 unum lattice that fits in 8 bits, where the reciprocals
 * and negatives are also exact finite decimals. Less accuracy wobble than binary IEEE floats. You
 * CAN have it all, folks!
 */
const BASIS:float = [ 1, 1.25, 1.6, 2, 2.5, 3.2, 4, 5, 6.4, 8, 10, 12.8, 16, 20, 25.6, 32, 40, 51.2,
    64, 80, 102.4, 128, 160, 204.8, 256, 320, 409.6, 512, 640, 819.2, 1024, 1280, 1638.4,
    2048, 2560, 3276.8, 4096, 5120, 6553.6, 8192, 10240, 13107.2, 16384, 20480, 26214.4,
    32768, 40960, 52428.8, 65536, 81920, 104858, 131072, 163840, 209715, 262144, 327680,
    419430, 524288, 655360, 1048576, 2097152, 4194304, 8388608, 16777216 ];
// TODO - express the basis a bit more reliably than _float_.

fn main() {
    println!("Hello, world!");
}
