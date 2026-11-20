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
# make ci-minimum-supported-rust-version: test with the minimum supported Rust version
##
.PHONY: ci-minimum-supported-rust-version
ci-minimum-supported-rust-version:
	bin/make-ci-minimum-supported-rust-version

##
# make test-benchmarks: run bin/test-benchmarks to generate code benchmarks with criterion.
##
.PHONY: test-benchmarks
test-benchmarks:
	bin/test-benchmarks

##
# make ci: run bin/ci to do continuous integration.
##
.PHONY: ci
ci:
	bin/ci

##
# make test-coverage: run bin/test-coverage to enforce 100% function, line, and region coverage.
##
.PHONY: test-coverage
test-coverage:
	bin/test-coverage

##
# make release VERSION=x.y.z: validate and publish an already-prepared release.
##
.PHONY: release
release:
	@test -n "$(VERSION)" || (echo "usage: make release VERSION=x.y.z" >&2; exit 2)
	bin/release "$(VERSION)"
