##
# help: Display this help message.
##
.PHONY: help
help:
        @awk '/^##/{a=1-a}a' $(MAKEFILE_LIST) | cut -c3-

# newline: This is a print helper when you want to embed a newline in a message.
define newline


endef

##
# make test: test everything
##
.PHONY: test
test:
	bin/test

##
# make test-minimum-supported-rust-version: test with the minimum supported Rust version
##
.PHONY: test-minimum-supported-rust-version
test-minimum-supported-rust-version:
	bin/test-minimum-supported-rust-version

##
# make benchmarks: run bin/benchmarks to generate code benchmarks with criterion.
##
.PHONY: benchmarks
benchmarks:
	bin/benchmarks

##
# make ci: run bin/ci to do continuous integration.
##
.PHONY: ci
ci:
	bin/ci

##
# make coverage: : run bin/coverage to generate code coverage analysis with tarpaulin.
##
.PHONY: coverage
coverage:
	bin/coverage
