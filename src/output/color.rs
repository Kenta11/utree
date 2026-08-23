// SPDX-License-Identifier: GPL-2.0-or-later
//! -C/-n colorization: a port of tree's parse_dir_colors() and color().

use crate::options::Options;

#[derive(Clone, Copy, PartialEq)]
pub enum Slot {
    Reset,
    Normal,
    File,
    Dir,
    Link,
    Fifo,
    Blk,
    Chr,
    Orphan,
    Sock,
    Setuid,
    Setgid,
    StickyOtherWritable,
    OtherWritable,
    Sticky,
    Exec,
    Missing,
    LeftCode,
    RightCode,
    EndCode,
}

const SLOTS: usize = 20;

// "do" (doors) is absent: unreachable on Linux, so entries are ignored.
const NAMES: [(&[u8], Slot); 19] = [
    (b"rs", Slot::Reset),
    (b"no", Slot::Normal),
    (b"fi", Slot::File),
    (b"di", Slot::Dir),
    (b"ln", Slot::Link),
    (b"pi", Slot::Fifo),
    (b"bd", Slot::Blk),
    (b"cd", Slot::Chr),
    (b"or", Slot::Orphan),
    (b"so", Slot::Sock),
    (b"su", Slot::Setuid),
    (b"sg", Slot::Setgid),
    (b"tw", Slot::StickyOtherWritable),
    (b"ow", Slot::OtherWritable),
    (b"st", Slot::Sticky),
    (b"ex", Slot::Exec),
    (b"mi", Slot::Missing),
    (b"lc", Slot::LeftCode),
    (b"rc", Slot::RightCode),
];

/// tree's builtin colors, used with CLICOLOR/CLICOLOR_FORCE and no
/// LS_COLORS.
const DEFAULT_COLORS: &[u8] = b":no=00:rs=0:fi=00:di=01;34:ln=01;36:pi=40;33:so=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:ex=01;32:*.bat=01;32:*.BAT=01;32:*.btm=01;32:*.BTM=01;32:*.cmd=01;32:*.CMD=01;32:*.com=01;32:*.COM=01;32:*.dll=01;32:*.DLL=01;32:*.exe=01;32:*.EXE=01;32:*.arj=01;31:*.bz2=01;31:*.deb=01;31:*.gz=01;31:*.lzh=01;31:*.rpm=01;31:*.tar=01;31:*.taz=01;31:*.tb2=01;31:*.tbz2=01;31:*.tbz=01;31:*.tgz=01;31:*.tz2=01;31:*.z=01;31:*.Z=01;31:*.zip=01;31:*.ZIP=01;31:*.zoo=01;31:*.asf=01;35:*.ASF=01;35:*.avi=01;35:*.AVI=01;35:*.bmp=01;35:*.BMP=01;35:*.flac=01;35:*.FLAC=01;35:*.gif=01;35:*.GIF=01;35:*.jpg=01;35:*.JPG=01;35:*.jpeg=01;35:*.JPEG=01;35:*.m2a=01;35:*.M2a=01;35:*.m2v=01;35:*.M2V=01;35:*.mov=01;35:*.MOV=01;35:*.mp3=01;35:*.MP3=01;35:*.mpeg=01;35:*.MPEG=01;35:*.mpg=01;35:*.MPG=01;35:*.ogg=01;35:*.OGG=01;35:*.ppm=01;35:*.rm=01;35:*.RM=01;35:*.tga=01;35:*.TGA=01;35:*.tif=01;35:*.TIF=01;35:*.wav=01;35:*.WAV=01;35:*.wmv=01;35:*.WMV=01;35:*.xbm=01;35:*.xpm=01;35:";

pub struct Colors {
    pub enabled: bool,
    /// ln=target: color link names like their targets.
    pub link_target: bool,
    codes: [Option<Vec<u8>>; SLOTS],
    /// Suffix rules in match order (last in the env string wins).
    exts: Vec<(Vec<u8>, Vec<u8>)>,
}

impl Colors {
    pub fn parse(opts: &Options) -> Colors {
        use std::io::IsTerminal;

        let mut colors = Colors {
            enabled: false,
            link_target: false,
            codes: Default::default(),
            exts: Vec::new(),
        };

        let mut nocolor = opts.no_color;
        if std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()) {
            nocolor = true;
        }
        if std::env::var_os("TERM").is_none() {
            return colors;
        }
        let clicolor = std::env::var_os("CLICOLOR").is_some();
        let mut force = opts.force_color;
        if std::env::var_os("CLICOLOR_FORCE").is_some() && !nocolor {
            force = true;
        }

        let env = std::env::var_os("TREE_COLORS").or_else(|| std::env::var_os("LS_COLORS"));
        let mut source: Option<Vec<u8>> = env.map(|v| {
            use std::os::unix::ffi::OsStrExt;
            v.as_bytes().to_vec()
        });
        if source.as_ref().is_none_or(|s| s.is_empty()) && (force || clicolor) {
            source = Some(DEFAULT_COLORS.to_vec());
        }
        let Some(source) = source else {
            return colors;
        };
        if !force && (nocolor || !std::io::stdout().is_terminal()) {
            return colors;
        }
        colors.enabled = true;

        for entry in source.split(|&c| c == b':') {
            let mut parts = entry.splitn(2, |&c| c == b'=');
            let key = parts.next().unwrap_or(b"");
            let value = parts.next();
            if key.is_empty() {
                continue;
            }
            if key[0] == b'*' {
                if let Some(value) = value {
                    // Parsed front-to-back but matched back-to-front.
                    colors.exts.insert(0, (key[1..].to_vec(), value.to_vec()));
                }
                continue;
            }
            if key == b"ln" && value.is_some_and(|v| v.eq_ignore_ascii_case(b"target")) {
                colors.link_target = true;
                colors.codes[Slot::Link as usize] = Some(b"01;36".to_vec());
                continue;
            }
            if let Some((_, slot)) = NAMES.iter().find(|(name, _)| *name == key)
                && let Some(value) = value
            {
                colors.codes[*slot as usize] = Some(value.to_vec());
            }
        }

        // Guaranteed defaults, assuming ANSI/vt100.
        let left = colors.codes[Slot::LeftCode as usize]
            .get_or_insert_with(|| b"\x1b[".to_vec())
            .clone();
        let right = colors.codes[Slot::RightCode as usize]
            .get_or_insert_with(|| b"m".to_vec())
            .clone();
        let reset = colors.codes[Slot::Reset as usize]
            .get_or_insert_with(|| b"0".to_vec())
            .clone();
        colors.codes[Slot::EndCode as usize].get_or_insert_with(|| [left, reset, right].concat());
        colors
    }

    fn seq(&self, slot: Slot) -> Option<Vec<u8>> {
        let code = self.codes[slot as usize].as_ref()?;
        Some(
            [
                self.codes[Slot::LeftCode as usize]
                    .as_deref()
                    .unwrap_or(b""),
                code,
                self.codes[Slot::RightCode as usize]
                    .as_deref()
                    .unwrap_or(b""),
            ]
            .concat(),
        )
    }

    pub fn end(&self) -> &[u8] {
        self.codes[Slot::EndCode as usize].as_deref().unwrap_or(b"")
    }

    /// tree's color(): the escape sequence for an entry, or None when
    /// it stays uncolored.
    pub fn for_entry(&self, mode: u32, name: &[u8], orphan: bool, islink: bool) -> Option<Vec<u8>> {
        const IFMT: u32 = 0o170000;

        if orphan {
            let slot = if islink { Slot::Missing } else { Slot::Orphan };
            if let Some(seq) = self.seq(slot) {
                return Some(seq);
            }
        }
        match mode & IFMT {
            0o010000 => self.seq(Slot::Fifo),
            0o020000 => self.seq(Slot::Chr),
            0o040000 => {
                if mode & 0o1000 != 0 {
                    if mode & 0o002 != 0
                        && let Some(seq) = self.seq(Slot::StickyOtherWritable)
                    {
                        return Some(seq);
                    }
                    if mode & 0o002 == 0
                        && let Some(seq) = self.seq(Slot::Sticky)
                    {
                        return Some(seq);
                    }
                }
                if mode & 0o002 != 0
                    && let Some(seq) = self.seq(Slot::OtherWritable)
                {
                    return Some(seq);
                }
                self.seq(Slot::Dir)
            }
            0o060000 => self.seq(Slot::Blk),
            0o120000 => self.seq(Slot::Link),
            0o140000 => self.seq(Slot::Sock),
            0o100000 => {
                if mode & 0o4000 != 0
                    && let Some(seq) = self.seq(Slot::Setuid)
                {
                    return Some(seq);
                }
                if mode & 0o2000 != 0
                    && let Some(seq) = self.seq(Slot::Setgid)
                {
                    return Some(seq);
                }
                if mode & 0o111 != 0
                    && let Some(seq) = self.seq(Slot::Exec)
                {
                    return Some(seq);
                }
                for (ext, code) in &self.exts {
                    let tail = if name.len() > ext.len() {
                        &name[name.len() - ext.len()..]
                    } else {
                        name
                    };
                    if tail == ext.as_slice() {
                        return Some(
                            [
                                self.codes[Slot::LeftCode as usize]
                                    .as_deref()
                                    .unwrap_or(b""),
                                code,
                                self.codes[Slot::RightCode as usize]
                                    .as_deref()
                                    .unwrap_or(b""),
                            ]
                            .concat(),
                        );
                    }
                }
                self.seq(Slot::File)
            }
            // Unknown types (e.g. a zeroed orphan-target mode) fall
            // back to "no", the normal color.
            _ => self.seq(Slot::Normal),
        }
    }
}
