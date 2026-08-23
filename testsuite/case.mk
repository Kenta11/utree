# Shared Makefile for every test case; each case's Makefile is a
# symlink to this file.
#   make check   run the case under both binaries and diff the outputs
#   make clean   remove the fixture and outputs

CASE := $(notdir $(CURDIR))
SUITE := $(abspath $(CURDIR)/..)

TREE_REF ?= $(abspath $(SUITE)/../reference/tree)
UTREE ?= $(abspath $(SUITE)/../target/debug/utree)

check:
	@if [ ! -x "$(TREE_REF)" ]; then \
	    $(MAKE) --no-print-directory -C "$(SUITE)" reference; \
	fi
	@FAILED=0; \
	TREE_BIN="$(TREE_REF)" TREE_OUT_SUFFIX=ref bash test.bash || FAILED=1; \
	TREE_BIN="$(UTREE)" TREE_OUT_SUFFIX=actual bash test.bash || FAILED=1; \
	if ! diff -ruN .ref .actual; then \
	    echo "FAILURE: $(CASE)"; \
	    FAILED=1; \
	fi; \
	exit $$FAILED

clean:
	@chmod -R u+rwX fixture 2>/dev/null; true
	rm -rf fixture .ref .actual

.PHONY: check clean
