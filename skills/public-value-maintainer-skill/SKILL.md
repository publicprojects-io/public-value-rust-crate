---
name: public-value-maintainer-skill
description: Maintain the `public-value` Rust crate — port a new topic, keep spec/architecture.md and spec/topics.md and llms.txt/llms.json in sync, and run the crate's full validation (clippy pedantic, tests, rustdoc). Use when the user asks to add a topic, update this crate's docs, bump its version, or review a change to src/ or tests/ for consistency with the rest of the crate.
---

# Maintaining the `public-value` crate

This skill is for changes to the crate itself, not for using it as a dependency (see
`public-value-skill` for that). Read `AGENTS.md` first — it is the terse, load-bearing version of
this skill's content and stays current with the crate's actual conventions.

## Adding or editing a topic

Follow `docs/tutorials/porting-a-new-topic.md` in full. In short: read the source topic doc
end-to-end, pick one of `spec/architecture.md`'s four modelling shapes, add the item to the right
`src/<module>.rs`, write a doctest reproducing a real worked-example number, add a test to the
matching `tests/comprehensive_group<N>.rs`, then validate:

```sh
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

All three must be clean. Don't consider a topic done on `cargo clippy --lib` alone — the
`--all-targets` pass catches doc-comment lints in `tests/*.rs` too.

## Keeping the index files in sync

Four files describe the same 64-item surface from different angles; a change to `src/` that adds,
renames, or removes a public item should update all that apply:

- **`spec/topics.md`** — the topic-to-module-to-shape mapping. Update the row for the topic you
  touched.
- **`README.md`** — the per-module public-item list under "Modules". Keep it a name list, not a
  description; the rustdoc is the description.
- **`llms.txt`** — the module list under "Modules" (descriptions only, not a full item list).
- **`llms.json`** — the `modules[].public_items` array. Validate it stays parseable JSON after
  editing (`python3 -c "import json; json.load(open('llms.json'))"` or equivalent).

`spec/architecture.md` only needs an update if you're changing a *convention* (a new shared
newtype, a new modelling shape, a new lint policy), not for an individual topic addition.

## Version and publish discipline

- Bump `Cargo.toml`'s `version` following semver: a new topic or function is a minor bump; a
  renamed or removed public item, or a changed function signature, is a major bump (this crate is
  pre-1.0, so under Cargo's convention a breaking change bumps the minor version instead, e.g.
  0.1.0 → 0.2.0).
- `cargo publish` is irreversible — a published version can be yanked but never reused or deleted.
  Run the full validation above, plus `cargo publish --dry-run`, before ever running the real
  command, and confirm with whoever owns the crate before the first publish or before publishing
  a version with a scope change (a new module, a changed dependency).
- Check `Cargo.toml`'s `license` field is one this crate is actually licensed under and that a
  license file matching it exists in the repository root before publishing — the `include` list
  references `LICENSE.md`, which does not ship by default and must be added deliberately.

## Reviewing someone else's change

Check for the failure modes this crate has hit before:

- A `Decimal` built from `Decimal::from_f64_retain` compared with `assert_eq!` instead of
  `.round_dp(n)` first — binary floating-point residue will make an otherwise-correct calculation
  fail its own test.
- A doctest or test number copied verbatim from the source doc without independently recomputing
  it — the source material has had at least two internal arithmetic inconsistencies found by
  recomputation (a distributional-weighting figure and a 50-year annuity factor). Recompute before
  trusting a source doc's stated approximation.
- A new `#[allow(clippy::...)]` without a `// why` comment — `spec/architecture.md` requires one.
