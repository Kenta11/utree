#!/usr/bin/env bash
# utree (release) vs the reference tree: hyperfine wall-clock and
# strace syscall counts.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REF="${UTREE_REF_TREE:-${ROOT}/reference/tree}"

if [[ ! -x "${REF}" ]]; then
    make -C "${ROOT}/testsuite" reference
fi

cargo build --release --manifest-path "${ROOT}/Cargo.toml"
UTREE="${ROOT}/target/release/utree"

# Materialize the criterion fixtures.
cargo bench --manifest-path "${ROOT}/Cargo.toml" --bench walk -- --test >/dev/null 2>&1 || true

export LC_ALL=C TREE_CHARSET=UTF-8
for fixture in wide deep bushy; do
    dir="${ROOT}/target/bench-fixtures/${fixture}"
    [[ -d "${dir}" ]] || continue
    echo "=== ${fixture} ==="
    if command -v hyperfine >/dev/null 2>&1; then
        hyperfine --warmup 3 --style basic \
            "${REF} ${dir}" "${UTREE} ${dir}"
    else
        echo "(hyperfine not installed; skipping wall-clock comparison)"
    fi
    if command -v strace >/dev/null 2>&1; then
        for bin in "${REF}" "${UTREE}"; do
            count=$(strace -f -c "${bin}" "${dir}" 2>&1 >/dev/null | awk 'END {print $4}')
            echo "syscalls ($(basename "${bin}")): ${count}"
        done
    fi
done
