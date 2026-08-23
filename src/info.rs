// SPDX-License-Identifier: GPL-2.0-or-later
//! --info / --infofile support: a port of tree's info.c. A `.info`
//! file holds pattern lines and tab-indented description blocks.

use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use crate::filter::gittrim;
use crate::pattern::patmatch;
use crate::walk::join_path;

struct InfoComment {
    patterns: Vec<Vec<u8>>,
    desc: Vec<Vec<u8>>,
}

struct InfoFile {
    base: Vec<u8>,
    comments: Vec<InfoComment>,
}

#[derive(Default)]
pub struct InfoStack {
    stack: Vec<InfoFile>,
}

fn parse_info_file(file: &Path) -> Option<Vec<InfoComment>> {
    let content = std::fs::read(file).ok()?;
    let mut comments = Vec::new();
    let mut patterns: Vec<Vec<u8>> = Vec::new();
    let mut desc: Vec<Vec<u8>> = Vec::new();

    for line in content.split(|&c| c == b'\n') {
        if line.first() == Some(&b'#') {
            continue;
        }
        let trimmed = gittrim(line);
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.first() == Some(&b'\t') {
            desc.push(trimmed[1..].to_vec());
        } else {
            if !desc.is_empty() {
                if !patterns.is_empty() {
                    comments.push(InfoComment {
                        patterns: std::mem::take(&mut patterns),
                        desc: std::mem::take(&mut desc),
                    });
                } else {
                    desc.clear();
                }
            }
            // tree strips a leading '/' (new_pattern).
            let pat = if trimmed.first() == Some(&b'/') {
                trimmed[1..].to_vec()
            } else {
                trimmed
            };
            patterns.push(pat);
        }
    }
    if !patterns.is_empty() {
        comments.push(InfoComment { patterns, desc });
    }
    Some(comments)
}

impl InfoStack {
    pub fn mark(&self) -> usize {
        self.stack.len()
    }

    pub fn truncate(&mut self, mark: usize) {
        self.stack.truncate(mark);
    }

    /// --infofile: push an explicit info file.
    pub fn push_file(&mut self, file: &Path) {
        if let Some(comments) = parse_info_file(file) {
            self.stack.push(InfoFile {
                base: file.as_os_str().as_bytes().to_vec(),
                comments,
            });
        }
    }

    /// Push `dir/.info` if present; the walk root also searches parent
    /// directories. Returns infocheck's `top` flag.
    pub fn push_dir(&mut self, dir: &Path, check_parents: bool) -> bool {
        let file = dir.join(".info");
        if file.is_file() {
            if let Some(comments) = parse_info_file(&file) {
                self.stack.push(InfoFile {
                    base: dir.as_os_str().as_bytes().to_vec(),
                    comments,
                });
                return true;
            }
            return false;
        }
        if check_parents {
            // The base stays the original directory even when the file
            // is found in a parent (tree: inf->path = scopy(path)).
            let mut rpath: PathBuf = dir.to_path_buf();
            while rpath != Path::new("/") {
                let Ok(parent) = std::fs::canonicalize(rpath.join("..")) else {
                    break;
                };
                rpath = parent;
                let candidate = rpath.join(".info");
                if candidate.is_file() {
                    if let Some(comments) = parse_info_file(&candidate) {
                        self.stack.push(InfoFile {
                            base: dir.as_os_str().as_bytes().to_vec(),
                            comments,
                        });
                        return true;
                    }
                    return false;
                }
            }
        }
        false
    }

    /// tree's infocheck: first matching description, innermost file
    /// first. `top` permits bare-name matches against the innermost
    /// file only.
    pub fn check(
        &self,
        path: &[u8],
        name: &[u8],
        mut top: bool,
        isdir: bool,
    ) -> Option<Vec<Vec<u8>>> {
        for inf in self.stack.iter().rev() {
            for com in &inf.comments {
                for pat in &com.patterns {
                    if patmatch(path, pat, isdir, false) == 1 {
                        return Some(com.desc.clone());
                    }
                    if top && patmatch(name, pat, isdir, false) == 1 {
                        return Some(com.desc.clone());
                    }
                    if patmatch(path, &join_path(&inf.base, pat), isdir, false) == 1 {
                        return Some(com.desc.clone());
                    }
                }
            }
            top = false;
        }
        None
    }
}
