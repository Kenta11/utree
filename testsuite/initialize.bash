#!/usr/bin/env bash
# -*- coding: utf-8 -*-
#
# Shared setup for test cases. make runs each test.bash twice via
# TREE_BIN / TREE_OUT_SUFFIX; a case only produces outputs.

# tree's sort order is locale-dependent; pin it (see COMPATIBILITY.md).
export LC_ALL=C
export TREE_CHARSET=UTF-8

if [[ -z ${home} ]]; then
    echo 'INTERNAL ERROR: ${home} is not set'
    caller
    exit 2
fi

tree="${TREE_BIN:-"${home}"/../../target/debug/utree}"
test_name=$(basename "${home}")
fixture="${home}"/fixture
actual="${home}"/."${TREE_OUT_SUFFIX:-out}"

# Each pass rebuilds the fixture, so inode- or time-dependent output
# must be pinned (touch -t) or masked by the test case itself.
# A crashed unreadable-dir run may leave 000-mode entries behind.
chmod -R u+rwX "${fixture}" 2>/dev/null
rm -rf "${actual}" "${fixture}"
mkdir -p "${actual}" "${fixture}"
cd "${fixture}"

echo "Test: ${test_name} (${TREE_OUT_SUFFIX:-out})"

# @fn fin
# @brief End of a test case.
function fin {
    exit 0
}
