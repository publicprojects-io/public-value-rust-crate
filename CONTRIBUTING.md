# Contributing to public-value

Thanks for helping improve this project.

## Ways to contribute

- **Report a problem:** open an issue describing what is wrong, unclear, or missing.
- **Fix or improve something:** open a pull request. Small fixes can go straight to a pull request; for larger changes, please open an issue first so we can agree on the approach.
- **Add content or examples:** follow the structure and style of the existing files.

## Development

Read `AGENTS.md` and `spec/` first; the spec is the source of truth. Layout: library modules in
`src/`, integration tests in `tests/`, tutorials in `docs/tutorials/`.

```sh
make ci                                    # runs bin/ci: fmt, clippy, tests + doctests, rustdoc
make ci-minimum-supported-rust-version   # check and test on the MSRV (1.98) and the next minor
```

Run `make ci` before opening a pull request. Every public item needs rustdoc with a doctest, and
clippy pedantic is denied crate-wide.

## New-topic quick reference

1. Read every file in `spec/`, then read the canonical source topic.
2. Add the slug and intended symbol to `spec/topics.md` before changing the public API.
3. Choose one of the four modelling shapes in `spec/architecture.md`.
4. Implement the item in its category module with `#[must_use]`, source credit, and a real worked
   example doctest.
5. Add the worked example to the matching `tests/comprehensive_groupN.rs` file.
6. Run `make ci`, `make test-coverage`, and the MSRV check.

The full rationale and walkthrough are in
[`docs/tutorials/porting-a-new-topic.md`](docs/tutorials/porting-a-new-topic.md).

## Code of conduct

Participation is governed by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contribution is offered under the terms in [LICENSE.md](LICENSE.md).
