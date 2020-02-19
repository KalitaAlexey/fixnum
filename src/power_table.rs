/*
    Script for generating the table content.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

import math

next_power_of_ten = lambda x: 10**math.ceil(math.log10(x))
values = [2**(i-1) for i in reversed(range(0,65))]
powers = [next_power_of_ten(value) for value in values]

print('//  lz |         value        | next power of ten')
print('//-----+----------------------+------------------')

for lz, (value, power) in enumerate(zip(values, powers)):
    if power > 9223372036854775807:
        print('/* %3d | %19d */ 0 /* overflow */,' % (lz, value))
    else:
        print('/* %3d | %19d */ %d,' % (lz, value, power))
*/

#[rustfmt::skip]
#[allow(clippy::all)]
pub static POWER_TABLE: [i64; 65] = [
    //  lz |         value        | next power of ten
    //-----+----------------------+------------------
    /*   0 | 9223372036854775808 */ 0 /* overflow */,
    /*   1 | 4611686018427387904 */ 0 /* overflow */,
    /*   2 | 2305843009213693952 */ 0 /* overflow */,
    /*   3 | 1152921504606846976 */ 0 /* overflow */,
    /*   4 |  576460752303423488 */ 1000000000000000000,
    /*   5 |  288230376151711744 */ 1000000000000000000,
    /*   6 |  144115188075855872 */ 1000000000000000000,
    /*   7 |   72057594037927936 */ 100000000000000000,
    /*   8 |   36028797018963968 */ 100000000000000000,
    /*   9 |   18014398509481984 */ 100000000000000000,
    /*  10 |    9007199254740992 */ 10000000000000000,
    /*  11 |    4503599627370496 */ 10000000000000000,
    /*  12 |    2251799813685248 */ 10000000000000000,
    /*  13 |    1125899906842624 */ 10000000000000000,
    /*  14 |     562949953421312 */ 1000000000000000,
    /*  15 |     281474976710656 */ 1000000000000000,
    /*  16 |     140737488355328 */ 1000000000000000,
    /*  17 |      70368744177664 */ 100000000000000,
    /*  18 |      35184372088832 */ 100000000000000,
    /*  19 |      17592186044416 */ 100000000000000,
    /*  20 |       8796093022208 */ 10000000000000,
    /*  21 |       4398046511104 */ 10000000000000,
    /*  22 |       2199023255552 */ 10000000000000,
    /*  23 |       1099511627776 */ 10000000000000,
    /*  24 |        549755813888 */ 1000000000000,
    /*  25 |        274877906944 */ 1000000000000,
    /*  26 |        137438953472 */ 1000000000000,
    /*  27 |         68719476736 */ 100000000000,
    /*  28 |         34359738368 */ 100000000000,
    /*  29 |         17179869184 */ 100000000000,
    /*  30 |          8589934592 */ 10000000000,
    /*  31 |          4294967296 */ 10000000000,
    /*  32 |          2147483648 */ 10000000000,
    /*  33 |          1073741824 */ 10000000000,
    /*  34 |           536870912 */ 1000000000,
    /*  35 |           268435456 */ 1000000000,
    /*  36 |           134217728 */ 1000000000,
    /*  37 |            67108864 */ 100000000,
    /*  38 |            33554432 */ 100000000,
    /*  39 |            16777216 */ 100000000,
    /*  40 |             8388608 */ 10000000,
    /*  41 |             4194304 */ 10000000,
    /*  42 |             2097152 */ 10000000,
    /*  43 |             1048576 */ 10000000,
    /*  44 |              524288 */ 1000000,
    /*  45 |              262144 */ 1000000,
    /*  46 |              131072 */ 1000000,
    /*  47 |               65536 */ 100000,
    /*  48 |               32768 */ 100000,
    /*  49 |               16384 */ 100000,
    /*  50 |                8192 */ 10000,
    /*  51 |                4096 */ 10000,
    /*  52 |                2048 */ 10000,
    /*  53 |                1024 */ 10000,
    /*  54 |                 512 */ 1000,
    /*  55 |                 256 */ 1000,
    /*  56 |                 128 */ 1000,
    /*  57 |                  64 */ 100,
    /*  58 |                  32 */ 100,
    /*  59 |                  16 */ 100,
    /*  60 |                   8 */ 10,
    /*  61 |                   4 */ 10,
    /*  62 |                   2 */ 10,
    /*  63 |                   1 */ 1,
    /*  64 |                   0 */ 1,
];