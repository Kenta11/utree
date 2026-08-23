// SPDX-License-Identifier: GPL-2.0-or-later
//! glibc's strverscmp state machine, as vendored by tree
//! (strverscmp.c), for -v / --sort=version.

const S_N: usize = 0x0;
const S_I: usize = 0x4;
const S_F: usize = 0x8;
const S_Z: usize = 0xC;

const CMP: i32 = 2;
const LEN: i32 = 3;

#[rustfmt::skip]
const NEXT_STATE: [usize; 16] = [
    /* state  x    d    0    - */
    /* S_N */ S_N, S_I, S_Z, S_N,
    /* S_I */ S_N, S_I, S_I, S_I,
    /* S_F */ S_N, S_F, S_F, S_F,
    /* S_Z */ S_N, S_F, S_Z, S_Z,
];

#[rustfmt::skip]
const RESULT_TYPE: [i32; 60] = [
    /* S_N */ CMP, CMP, CMP, CMP,  CMP, LEN, CMP, CMP,
              CMP, CMP, CMP, CMP,  CMP, CMP, CMP, CMP,
    /* S_I */ CMP, -1,  -1,  CMP,   1,  LEN, LEN, CMP,
               1,  LEN, LEN, CMP,  CMP, CMP, CMP, CMP,
    /* S_F */ CMP, CMP, CMP, CMP,  CMP, LEN, CMP, CMP,
              CMP, CMP, CMP, CMP,  CMP, CMP, CMP, CMP,
    /* S_Z */ CMP,  1,   1,  CMP,  -1,  CMP, CMP, CMP,
              -1,  CMP, CMP, CMP,
];

pub fn strverscmp(s1: &[u8], s2: &[u8]) -> i32 {
    let at = |s: &[u8], i: usize| -> u8 { if i < s.len() { s[i] } else { 0 } };
    let class = |c: u8| -> usize { usize::from(c == b'0') + usize::from(c.is_ascii_digit()) };

    let (mut i1, mut i2) = (0usize, 0usize);
    let mut c1 = at(s1, i1);
    let mut c2 = at(s2, i2);
    i1 += 1;
    i2 += 1;
    let mut state = S_N | class(c1);

    let mut diff = i32::from(c1) - i32::from(c2);
    while diff == 0 && c1 != 0 {
        state = NEXT_STATE[state];
        c1 = at(s1, i1);
        c2 = at(s2, i2);
        i1 += 1;
        i2 += 1;
        state |= class(c1);
        diff = i32::from(c1) - i32::from(c2);
    }

    // The full glibc table covers S_Z rows, but S_F/S_Z entries beyond
    // the ones reachable here are all CMP; index defensively.
    let idx = (state << 2) | class(c2);
    let state = if idx < RESULT_TYPE.len() {
        RESULT_TYPE[idx]
    } else {
        CMP
    };

    match state {
        CMP => diff,
        LEN => {
            while at(s1, i1).is_ascii_digit() {
                i1 += 1;
                if !at(s2, i2).is_ascii_digit() {
                    return 1;
                }
                i2 += 1;
            }
            if at(s2, i2).is_ascii_digit() {
                -1
            } else {
                diff
            }
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::strverscmp;

    #[test]
    fn glibc_documented_cases() {
        assert_eq!(strverscmp(b"no digit", b"no digit"), 0);
        assert!(strverscmp(b"item#99", b"item#100") < 0);
        assert!(strverscmp(b"alpha1", b"alpha001") > 0);
        assert!(strverscmp(b"part1_f012", b"part1_f01") > 0);
        assert!(strverscmp(b"foo.009", b"foo.0") < 0);
        assert!(strverscmp(b"z2", b"z10") < 0);
    }
}
