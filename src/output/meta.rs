// SPDX-License-Identifier: GPL-2.0-or-later
//! The bracketed metadata column (tree's fillinfo/prot/psize/do_date)
//! and -F classification characters.

use std::collections::HashMap;

use crate::options::Options;
use crate::walk::Meta;

const S_IFMT: u32 = 0o170000;
const TYPES: [(u32, u8); 7] = [
    (0o100000, b'-'),
    (0o040000, b'd'),
    (0o120000, b'l'),
    (0o020000, b'c'),
    (0o060000, b'b'),
    (0o140000, b's'),
    (0o010000, b'p'),
];

/// tree's prot(): "drwxr-xr-x" style, with suid/sgid/sticky.
pub fn prot(mode: u32) -> [u8; 10] {
    let mut buf = [b'-'; 10];
    buf[0] = TYPES
        .iter()
        .find(|(fmt, _)| mode & S_IFMT == *fmt)
        .map_or(b'?', |(_, c)| *c);
    let perms = b"rwxrwxrwx";
    for (i, &p) in perms.iter().enumerate() {
        if mode & (0o400 >> i) != 0 {
            buf[i + 1] = p;
        }
    }
    if mode & 0o4000 != 0 {
        buf[3] = if buf[3] == b'-' { b'S' } else { b's' };
    }
    if mode & 0o2000 != 0 {
        buf[6] = if buf[6] == b'-' { b'S' } else { b's' };
    }
    if mode & 0o1000 != 0 {
        buf[9] = if buf[9] == b'-' { b'T' } else { b't' };
    }
    buf
}

/// tree's psize(); the float math mirrors the C exactly, quirky
/// (size+52) rounding-mode choice included.
pub fn psize(size: i64, opts: &Options) -> Vec<u8> {
    if opts.human || opts.si {
        let base: i64 = if opts.si { 1000 } else { 1024 };
        let units: &[u8] = if opts.si { b"dkMGTPEZY" } else { b"BKMGTPEZY" };
        let mut idx = usize::from(size >= base);
        let mut size = size;
        while size >= base * base {
            idx += 1;
            size /= base;
        }
        if idx == 0 {
            format!(" {size:4}").into_bytes()
        } else {
            let value = (size as f32) / (base as f32);
            let unit = units[idx] as char;
            if (size + 52) / base >= 10 {
                format!(" {value:3.0}{unit}").into_bytes()
            } else {
                format!(" {value:3.1}{unit}").into_bytes()
            }
        }
    } else {
        format!(" {size:11}").into_bytes()
    }
}

const SIXMONTHS: i64 = 6 * 31 * 24 * 60 * 60;

/// tree's do_date(): --timefmt, else "%b %e  %Y" for old files and
/// "%b %e %R" for recent ones. Invalid --timefmt specifiers fall back
/// to the raw format string (chrono's Display would panic on them).
pub fn do_date(t: i64, opts: &Options, now: i64) -> Vec<u8> {
    use chrono::TimeZone;
    let Some(dt) = chrono::Local.timestamp_opt(t, 0).single() else {
        return Vec::new();
    };
    let fmt = match &opts.timefmt {
        Some(fmt) => String::from_utf8_lossy(fmt).into_owned(),
        None => {
            if t > now || t + SIXMONTHS < now {
                "%b %e  %Y".to_string()
            } else {
                "%b %e %R".to_string()
            }
        }
    };
    let items: Vec<_> = chrono::format::StrftimeItems::new(&fmt).collect();
    if items
        .iter()
        .any(|i| matches!(i, chrono::format::Item::Error))
    {
        return fmt.into_bytes();
    }
    format!(
        "{}",
        dt.format_with_items(chrono::format::StrftimeItems::new(&fmt))
    )
    .into_bytes()
}

/// uid/gid to name, numeric fallback — tree uses getpwuid/getgrgid;
/// utree reads /etc/passwd and /etc/group directly (no NSS), see
/// COMPATIBILITY.md.
#[derive(Default)]
pub struct Ids {
    users: Option<HashMap<u32, Vec<u8>>>,
    groups: Option<HashMap<u32, Vec<u8>>>,
}

fn parse_id_file(path: &str) -> HashMap<u32, Vec<u8>> {
    let mut map = HashMap::new();
    let Ok(content) = std::fs::read(path) else {
        return map;
    };
    for line in content.split(|&c| c == b'\n') {
        let mut fields = line.split(|&c| c == b':');
        let (Some(name), Some(_), Some(id)) = (fields.next(), fields.next(), fields.next()) else {
            continue;
        };
        let Ok(id) = std::str::from_utf8(id).unwrap_or("").parse::<u32>() else {
            continue;
        };
        map.entry(id).or_insert_with(|| name.to_vec());
    }
    map
}

impl Ids {
    pub fn user(&mut self, uid: u32) -> Vec<u8> {
        self.users
            .get_or_insert_with(|| parse_id_file("/etc/passwd"))
            .get(&uid)
            .cloned()
            .unwrap_or_else(|| uid.to_string().into_bytes())
    }

    pub fn group(&mut self, gid: u32) -> Vec<u8> {
        self.groups
            .get_or_insert_with(|| parse_id_file("/etc/group"))
            .get(&gid)
            .cloned()
            .unwrap_or_else(|| gid.to_string().into_bytes())
    }
}

/// " %-8.32s": left-justified to at least 8, truncated at 32.
fn pad_name(name: &[u8]) -> Vec<u8> {
    let name = &name[..name.len().min(32)];
    let mut out = Vec::with_capacity(9);
    out.push(b' ');
    out.extend_from_slice(name);
    while out.len() < 9 {
        out.push(b' ');
    }
    out
}

pub fn wants_info(opts: &Options) -> bool {
    opts.show_inode
        || opts.show_device
        || opts.show_perms
        || opts.show_uid
        || opts.show_gid
        || opts.show_size
        || opts.show_date
}

/// tree's fillinfo(): the "[...]" block, or None when no flag asks
/// for it.
pub fn fillinfo(meta: Option<&Meta>, opts: &Options, ids: &mut Ids, now: i64) -> Option<Vec<u8>> {
    if !wants_info(opts) {
        return None;
    }
    let meta = meta?;
    let mut buf = Vec::new();
    if opts.show_inode {
        buf.extend_from_slice(format!(" {:7}", meta.ino).as_bytes());
    }
    if opts.show_device {
        buf.extend_from_slice(format!(" {:3}", meta.dev as i32).as_bytes());
    }
    if opts.show_perms {
        buf.push(b' ');
        buf.extend_from_slice(&prot(meta.mode));
    }
    if opts.show_uid {
        buf.extend_from_slice(&pad_name(&ids.user(meta.uid)));
    }
    if opts.show_gid {
        buf.extend_from_slice(&pad_name(&ids.group(meta.gid)));
    }
    if opts.show_size {
        buf.extend_from_slice(&psize(meta.size, opts));
    }
    if opts.show_date {
        buf.push(b' ');
        let t = if opts.use_ctime {
            meta.ctime
        } else {
            meta.mtime
        };
        buf.extend_from_slice(&do_date(t, opts, now));
    }
    if buf.first() == Some(&b' ') {
        buf[0] = b'[';
        buf.push(b']');
    }
    Some(buf)
}

/// tree's Ftype() for -F.
pub fn ftype(mode: u32, opts: &Options) -> Option<u8> {
    match mode & S_IFMT {
        0o040000 if !opts.dirs_only => Some(b'/'),
        0o140000 => Some(b'='),
        0o010000 => Some(b'|'),
        0o120000 => Some(b'@'),
        0o100000 if mode & 0o111 != 0 => Some(b'*'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::prot;

    #[test]
    fn prot_formats_modes() {
        assert_eq!(&prot(0o100644), b"-rw-r--r--");
        assert_eq!(&prot(0o040755), b"drwxr-xr-x");
        assert_eq!(&prot(0o104755), b"-rwsr-xr-x");
        assert_eq!(&prot(0o120777), b"lrwxrwxrwx");
        assert_eq!(&prot(0o041777), b"drwxrwxrwt");
    }
}
