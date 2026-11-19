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
	cargo test --all-targets --all-features
	cargo test --doc --all-features

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
# make coverage: run bin/coverage to enforce 100% function, line, and region coverage.
##
.PHONY: coverage
coverage:
	bin/coverage

##
# make release VERSION=x.y.z: validate and publish an already-prepared release.
##
.PHONY: release
release:
	@test -n "$(VERSION)" || (echo "usage: make release VERSION=x.y.z" >&2; exit 2)
	bin/release "$(VERSION)"
