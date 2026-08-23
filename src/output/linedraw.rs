// SPDX-License-Identifier: GPL-2.0-or-later
//! Line-drawing glyphs: tree's initlinedraw() and its full cstable.

use crate::options::Options;

pub struct LineDraw {
    /// Continuation under a branch with more siblings.
    pub vert: &'static [u8],
    pub tee: &'static [u8],
    pub corner: &'static [u8],
    /// --info comment brackets: multi-line top/bottom/middle/extension
    /// and the single-line form.
    pub ctop: &'static [u8],
    pub cbot: &'static [u8],
    pub cmid: &'static [u8],
    pub cext: &'static [u8],
    pub csingle: &'static [u8],
    /// The copyright glyph for version banners (tree's linedraw->copy).
    pub copy: &'static [u8],
}

const PLAIN_BRACKETS: [&[u8]; 5] = [b" [", b" [", b" [", b" [", b" ["];

macro_rules! linedraw {
    ($vert:expr, $tee:expr, $corner:expr, $copy:expr) => {
        LineDraw {
            vert: $vert,
            tee: $tee,
            corner: $corner,
            ctop: PLAIN_BRACKETS[0],
            cbot: PLAIN_BRACKETS[1],
            cmid: PLAIN_BRACKETS[2],
            cext: PLAIN_BRACKETS[3],
            csingle: PLAIN_BRACKETS[4],
            copy: $copy,
        }
    };
}

pub const UTF8: LineDraw = LineDraw {
    vert: "\u{2502}\u{a0}\u{a0}".as_bytes(),
    tee: "\u{251c}\u{2500}\u{2500}".as_bytes(),
    corner: "\u{2514}\u{2500}\u{2500}".as_bytes(),
    ctop: " \u{23a7}".as_bytes(),
    cbot: " \u{23a9}".as_bytes(),
    cmid: " \u{23a8}".as_bytes(),
    cext: " \u{23aa}".as_bytes(),
    csingle: " {".as_bytes(),
    copy: b"\xc2\xa9",
};

pub const ASCII: LineDraw = linedraw!(b"|  ", b"|--", b"`--", b"(c)");

/// -A: vt100 alternate-charset graphics.
pub const ANSI: LineDraw = linedraw!(
    b"\x1b(0\x78  \x1b(B",
    b"\x1b(0\x74\x71\x71\x1b(B",
    b"\x1b(0\x6d\x71\x71\x1b(B",
    b"\x1b(0\xa9\x1b(B"
);

const LATIN1_3: LineDraw = linedraw!(b"|  ", b"|--", b"&middot;--", b"&copy;");
const ISO8859_789: LineDraw = linedraw!(b"|  ", b"|--", b"&middot;--", b"(c)");
const SHIFT_JIS: LineDraw = linedraw!(
    b"\x84\xa0  ",
    b"\x84\xa5\x84\x9f\x84\x9f",
    b"\x84\xa4\x84\x9f\x84\x9f",
    b"(c)"
);
const EUC_JP: LineDraw = linedraw!(
    b"\xa8\xa2  ",
    b"\xa8\xa7\xa8\xa1\xa8\xa1",
    b"\xa8\xa6\xa8\xa1\xa8\xa1",
    b"(c)"
);
const EUC_KR: LineDraw = linedraw!(
    b"\xa6\xa2  ",
    b"\xa6\xa7\xa6\xa1\xa6\xa1",
    b"\xa6\xa6\xa6\xa1\xa6\xa1",
    b"(c)"
);
const ISO2022JP: LineDraw = linedraw!(
    b"\x1b$B(\"\x1b(B  ",
    b"\x1b$B('\x1b$B(!\x1b$B(!\x1b(B",
    b"\x1b$B(&\x1b$B(!\x1b$B(!\x1b(B",
    b"(c)"
);
/// -S / --charset=IBM437: CP437 console graphics.
pub const IBM437: LineDraw = linedraw!(b"\xb3  ", b"\xc3\xc4\xc4", b"\xc0\xc4\xc4", b"(c)");
const IBM_PS2: LineDraw = linedraw!(b"\xb3  ", b"\xc3\xc4\xc4", b"\xc0\xc4\xc4", b"\x97");
const IBM_GR: LineDraw = linedraw!(b"\xb3  ", b"\xc3\xc4\xc4", b"\xc0\xc4\xc4", b"\xb8");
const GB: LineDraw = linedraw!(
    b"\xa9\xa6  ",
    b"\xa9\xc0\xa9\xa4\xa9\xa4",
    b"\xa9\xb8\xa9\xa4\xa9\xa4",
    b"(c)"
);
const BIG5: LineDraw = linedraw!(b"\xa2x  ", b"\xa2u\xa2w\xa2w", b"\xa2|\xa2w\xa2w", b"(c)");
const VISCII: LineDraw = linedraw!(b"|  ", b"|--", b"`--", b"\xf9");
const KOI8: LineDraw = linedraw!(b"\x81  ", b"\x86\x80\x80", b"\x84\x80\x80", b"\xbf");
const WINDOWS: LineDraw = linedraw!(b"|  ", b"|--", b"`--", b"\xa9");

/// cstable: rows in tree's order, matched case-insensitively; the
/// first alias hit wins.
const CSTABLE: [(&[&[u8]], &LineDraw); 16] = [
    (&[b"ANSI"], &ANSI),
    (
        &[
            b"ISO-8859-1",
            b"ISO-8859-1:1987",
            b"ISO_8859-1",
            b"latin1",
            b"l1",
            b"IBM819",
            b"CP819",
            b"csISOLatin1",
            b"ISO-8859-3",
            b"ISO_8859-3:1988",
            b"ISO_8859-3",
            b"latin3",
            b"ls",
            b"csISOLatin3",
        ],
        &LATIN1_3,
    ),
    (
        &[
            b"ISO-8859-7",
            b"ISO_8859-7:1987",
            b"ISO_8859-7",
            b"ELOT_928",
            b"ECMA-118",
            b"greek",
            b"greek8",
            b"csISOLatinGreek",
            b"ISO-8859-8",
            b"ISO_8859-8:1988",
            b"iso-ir-138",
            b"ISO_8859-8",
            b"hebrew",
            b"csISOLatinHebrew",
            b"ISO-8859-9",
            b"ISO_8859-9:1989",
            b"iso-ir-148",
            b"ISO_8859-9",
            b"latin5",
            b"l5",
            b"csISOLatin5",
        ],
        &ISO8859_789,
    ),
    (&[b"Shift_JIS", b"MS_Kanji", b"csShiftJIS"], &SHIFT_JIS),
    (
        &[
            b"EUC-JP",
            b"Extended_UNIX_Code_Packed_Format_for_Japanese",
            b"csEUCPkdFmtJapanese",
        ],
        &EUC_JP,
    ),
    (&[b"EUC-KR", b"csEUCKR"], &EUC_KR),
    (
        &[
            b"ISO-2022-JP",
            b"csISO2022JP",
            b"ISO-2022-JP-2",
            b"csISO2022JP2",
        ],
        &ISO2022JP,
    ),
    (
        &[
            b"IBM437",
            b"cp437",
            b"437",
            b"csPC8CodePage437",
            b"IBM852",
            b"cp852",
            b"852",
            b"csPCp852",
            b"IBM863",
            b"cp863",
            b"863",
            b"csIBM863",
            b"IBM855",
            b"cp855",
            b"855",
            b"csIBM855",
            b"IBM865",
            b"cp865",
            b"865",
            b"csIBM865",
            b"IBM866",
            b"cp866",
            b"866",
            b"csIBM866",
        ],
        &IBM437,
    ),
    (
        &[
            b"IBM850",
            b"cp850",
            b"850",
            b"csPC850Multilingual",
            b"IBM00858",
            b"CCSID00858",
            b"CP00858",
            b"PC-Multilingual-850+euro",
        ],
        &IBM_PS2,
    ),
    (
        &[b"IBM869", b"cp869", b"869", b"cp-gr", b"csIBM869"],
        &IBM_GR,
    ),
    (&[b"GB2312", b"csGB2312"], &GB),
    (&[b"UTF-8", b"utf8"], &UTF8),
    (&[b"Big5", b"csBig5"], &BIG5),
    (&[b"VISCII", b"csVISCII"], &VISCII),
    (&[b"KOI8-R", b"csKOI8R", b"KOI8-U"], &KOI8),
    (
        &[
            b"ISO-8859-1-Windows-3.1-Latin-1",
            b"csWindows31Latin1",
            b"ISO-8859-2-Windows-Latin-2",
            b"csWindows31Latin2",
            b"windows-1250",
            b"windows-1251",
            b"windows-1253",
            b"windows-1254",
            b"windows-1255",
            b"windows-1256",
            b"windows-1256",
            b"windows-1257",
        ],
        &WINDOWS,
    ),
];

fn by_name(name: &[u8]) -> &'static LineDraw {
    for (aliases, draw) in &CSTABLE {
        if aliases.iter().any(|a| a.eq_ignore_ascii_case(name)) {
            return draw;
        }
    }
    &ASCII
}

/// The charset NAME (for XML encoding= and the HTML meta tag):
/// --charset, TREE_CHARSET, or the locale's UTF-8.
pub fn charset_name(opts: &Options) -> Option<Vec<u8>> {
    use std::os::unix::ffi::OsStrExt;

    if let Some(name) = &opts.charset {
        return Some(name.clone());
    }
    if let Some(env) = std::env::var_os("TREE_CHARSET") {
        return Some(env.as_bytes().to_vec());
    }
    for var in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Some(value) = std::env::var_os(var) {
            if value.is_empty() {
                continue;
            }
            let upper = value.as_bytes().to_ascii_uppercase();
            let utf8 =
                upper.windows(5).any(|w| w == b"UTF-8") || upper.windows(4).any(|w| w == b"UTF8");
            return utf8.then(|| b"UTF-8".to_vec());
        }
    }
    None
}

/// Priority mirrors tree: -A, then --charset, then TREE_CHARSET, then
/// the locale (approximating nl_langinfo(CODESET) via the environment).
pub fn select(opts: &Options) -> &'static LineDraw {
    use std::os::unix::ffi::OsStrExt;

    // -A wins over any charset (initlinedraw checks it first).
    if opts.ansilines {
        return &ANSI;
    }
    let explicit = opts
        .charset
        .clone()
        .or_else(|| std::env::var_os("TREE_CHARSET").map(|v| v.as_bytes().to_vec()));
    if let Some(name) = explicit {
        return by_name(&name);
    }
    for var in ["LC_ALL", "LC_CTYPE", "LANG"] {
        if let Some(value) = std::env::var_os(var) {
            if value.is_empty() {
                continue;
            }
            let upper = value.as_bytes().to_ascii_uppercase();
            let utf8 =
                upper.windows(5).any(|w| w == b"UTF-8") || upper.windows(4).any(|w| w == b"UTF8");
            return if utf8 { &UTF8 } else { &ASCII };
        }
    }
    &ASCII
}
