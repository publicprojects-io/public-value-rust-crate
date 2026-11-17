.PHONY: test-minimum-supported-rust-version

test-minimum-supported-rust-version:
	bin/make-test-minimum-supported-rust-version

.PHONY: ci

ci:
	bin/ci
