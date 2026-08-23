# Differential compatibility suite. The reference tree is built from
# the tree/ submodule on demand (`make reference`).
#   make check         run the cases utree implements (the default)
#   make check-all     run every case, unimplemented ones included
#   make -C <case> check   run a single case

# A test case is any directory containing a test.bash.
TEST_ENTRIES := $(sort $(patsubst %/test.bash,%,$(wildcard */test.bash)))

# Skipped by check; check-all runs everything. Currently empty.
UNIMPLEMENTED = \
	$(NULL)

check: TEST_ENTRIES := $(filter-out $(UNIMPLEMENTED),$(TEST_ENTRIES))
check: check-all

check-all:
	@echo "***** tests are launched ******"
	@FAILED=0; \
	for ENTRY in $(TEST_ENTRIES); do \
	    $(MAKE) --no-print-directory -C $${ENTRY} check || FAILED=1; \
	done; \
	echo "***** tests were finished: $${FAILED} ******"; exit "$${FAILED}"

clean:
	@for ENTRY in $(TEST_ENTRIES); do \
	    $(MAKE) --no-print-directory -C $${ENTRY} clean; \
	done

# Build the reference binary inside the submodule at the repo root.
reference: ../reference/tree

../reference/tree:
	@if [ ! -f ../reference/Makefile ]; then git -C .. submodule update --init reference; fi
	@$(MAKE) --no-print-directory -C ../reference >/dev/null

.PHONY: check check-all clean reference
