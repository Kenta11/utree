// SPDX-License-Identifier: GPL-2.0-or-later
//! Output backends: every format is a `Formatter` fed the same tree.

pub mod color;
pub mod html;
pub mod json;
pub mod linedraw;
pub mod meta;
pub mod text;
pub mod xml;

pub(crate) fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

use std::io::{self, Write};

use crate::options::Options;
use crate::walk::Root;

#[derive(Debug, Default, Clone, Copy)]
pub struct Totals {
    pub dirs: u64,
    pub files: u64,
    /// --du bytes (accumulated directory sizes).
    pub size: i64,
}

impl Totals {
    pub fn add(&mut self, other: &Totals) {
        self.dirs += other.dirs;
        self.files += other.files;
        self.size += other.size;
    }
}

pub trait Formatter {
    fn intro(&mut self, _out: &mut dyn Write, _opts: &Options) -> io::Result<()> {
        Ok(())
    }
    fn emit_root(
        &mut self,
        out: &mut dyn Write,
        root: &Root,
        opts: &Options,
        more_roots: bool,
    ) -> io::Result<()>;
    fn emit_report(
        &mut self,
        out: &mut dyn Write,
        totals: &Totals,
        opts: &Options,
    ) -> io::Result<()>;
    fn outro(&mut self, _out: &mut dyn Write, _opts: &Options) -> io::Result<()> {
        Ok(())
    }
}
