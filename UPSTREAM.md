# Upstream pull requests and utree

Every pull request on [Old-Man-Programmer/tree](https://github.com/Old-Man-Programmer/tree/pulls) and merge request on [OldManProgrammer/unix-tree](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests), with utree's position on each. Status values:

- **fixed** — the fix is applied to utree and carried by the reference pin (see COMPATIBILITY.md, "Fixed relative to v2.3.2")
- **immune** — the bug cannot occur in utree by construction (memory safety, no shared mutable state)
- **mirrors** — utree currently reproduces the upstream behavior
- **inherited** — merged upstream before v2.3.2; utree ports the fixed behavior as part of the baseline
- **n/a** — does not concern utree (feature proposal, C-specific cleanup, build/infra, platforms, unimplemented options)

## GitHub pull requests

| PR | State | Subject | utree |
|---|---|---|---|
| [#52](https://github.com/Old-Man-Programmer/tree/pull/52) | open | -R sub-listing indent state | fixed |
| [#51](https://github.com/Old-Man-Programmer/tree/pull/51) | open | full-tree walk exit status | fixed |
| [#50](https://github.com/Old-Man-Programmer/tree/pull/50) | open | glob syntax error counts as match | fixed |
| [#49](https://github.com/Old-Man-Programmer/tree/pull/49) | open | -J trailing comma, unopenable root | fixed |
| [#48](https://github.com/Old-Man-Programmer/tree/pull/48) | open | -J missing comma after empty root | fixed |
| [#47](https://github.com/Old-Man-Programmer/tree/pull/47) | open | -J spurious "contents" after error | fixed |
| [#42](https://github.com/Old-Man-Programmer/tree/pull/42) | open | test framework | n/a — utree has its own differential suite |
| [#41](https://github.com/Old-Man-Programmer/tree/pull/41) | open | --infofile absolute-path patterns | mirrors — adoption candidate, fix verified to apply cleanly |
| [#39](https://github.com/Old-Man-Programmer/tree/pull/39) | open | --stats flag | n/a — feature proposal |
| [#37](https://github.com/Old-Man-Programmer/tree/pull/37) | open | remove C99 code | n/a — C cleanup |
| [#36](https://github.com/Old-Man-Programmer/tree/pull/36) | open | const qualifier warning | n/a — C cleanup |
| [#35](https://github.com/Old-Man-Programmer/tree/pull/35) | closed | EXIT_SUCCESS/EXIT_FAILURE | n/a — applied in 2.3.2, no behavior change |
| [#31](https://github.com/Old-Man-Programmer/tree/pull/31) | closed | const qualifier warning | n/a — C cleanup |
| [#28](https://github.com/Old-Man-Programmer/tree/pull/28) | open | --focus-root feature | n/a — feature proposal |
| [#27](https://github.com/Old-Man-Programmer/tree/pull/27) | closed | codespell | n/a — infra |
| [#25](https://github.com/Old-Man-Programmer/tree/pull/25) | closed | zig build system | n/a — infra |
| [#21](https://github.com/Old-Man-Programmer/tree/pull/21) | closed | NULL check in unix_printfile | immune — no null pointers |
| [#20](https://github.com/Old-Man-Programmer/tree/pull/20) | closed | markdown output, word count | n/a — feature proposal |
| [#19](https://github.com/Old-Man-Programmer/tree/pull/19) | open | GitHub CI | n/a — infra |
| [#12](https://github.com/Old-Man-Programmer/tree/pull/12) | open | z/OS support | n/a — platform |
| [#9](https://github.com/Old-Man-Programmer/tree/pull/9) | closed | OSC 8 hyperlinks | n/a — --hyperlink unimplemented (COMPATIBILITY.md) |
| [#6](https://github.com/Old-Man-Programmer/tree/pull/6) | open | Android NDK build | n/a — platform |
| [#4](https://github.com/Old-Man-Programmer/tree/pull/4) | closed | JSON error comma | fixed — superseded by #49 |
| [#3](https://github.com/Old-Man-Programmer/tree/pull/3) | closed | --gitignore fix | inherited |

## GitLab merge requests

| MR | State | Subject | utree |
|---|---|---|---|
| [!31](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/31) | open | README https links | n/a — docs |
| [!30](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/30) | open | exit status documentation | n/a — docs/refactor; the behavior half is #51 |
| [!29](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/29) | open | macros to inline functions | n/a — C cleanup |
| [!28](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/28) | open | time_t portability | n/a — no behavior change; utree uses chrono |
| [!27](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/27) | closed | wrong variable freed | immune |
| [!26](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/26) | closed | baseHREF offset in HTML | inherited |
| [!25](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/25) | closed | Makefile packaging | n/a — build |
| [!24](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/24) | closed | man SEE ALSO | n/a — docs |
| [!23](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/23) | open | individual headers | n/a — C cleanup |
| [!22](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/22) | closed | gcc warning flags | n/a — build |
| [!21](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/21) | open | POSIX.1-2001 standardization | n/a — C cleanup |
| [!20](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/20) | open | remove unnecessary code | n/a — dead code only |
| [!19](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/19) | open | test framework | n/a — utree has its own differential suite |
| [!18](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/18) | closed | directory count fix | inherited |
| [!17](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/17) | closed | add .gitignore | n/a — infra |
| [!16](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/16) | closed | -R with -H links (issue #28) | inherited |
| [!15](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/15) | closed | free unfreed dirname | immune |
| [!14](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/14) | closed | use Unix toolbox | n/a — rejected refactor |
| [!13](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/13) | open | --fromfile buffer overflow | n/a — --fromfile unimplemented; the bug class is also immune |
| [!12](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/12) | closed | issue templates | n/a — infra |
| [!11](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/11) | closed | ignorefile parent search | inherited |
| [!10](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/10) | closed | older compiler support | n/a — build |
| [!9](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/9) | closed | ENV34-C fix | n/a — C cleanup |
| [!8](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/8) | closed | MD5 support (issue 8) | n/a — feature proposal |
| [!7](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/7) | closed | incomplete usage text | n/a — utree has its own --help |
| [!6](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/6) | closed | misc patch | n/a |
| [!5](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/5) | closed | reset dir after free | immune |
| [!4](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/4) | closed | clear pointer after free_dir | immune |
| [!3](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/3) | closed | normalize host for HTML | inherited |
| [!2](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/2) | closed | flimit type fix | immune — typed as a signed 64-bit count |
| [!1](https://gitlab.com/OldManProgrammer/unix-tree/-/merge_requests/1) | closed | CFLAGS/LDFLAGS from env | n/a — build |
