# Backlog resolution

Resolved 2026-11-19 against `spec/architecture.md`, `spec/topics.md`, and the current repository.
“Declined” is a completed product decision, not deferred work: the recommendation conflicts with
the specification, duplicates a stronger existing mechanism, lacks truthful external data, or
would add complexity without a source requirement.

## Onboarding, documentation, and promotion

| IDs | Resolution |
|---|---|
| ONB-001–003 | Completed in the getting-started tutorial and README quick examples/module tour. |
| ONB-004 | Completed: the demo runs six calculations and supports `--help`; integration-tested. |
| ONB-005 | Completed in `FAQ.md`. |
| DOC-001 | Completed: every category and units module has a runnable module-level example. |
| DOC-002 | Completed by the GitHub Pages documentation workflow. |
| DOC-003 | Completed by the six-step CONTRIBUTING quick reference. |
| DOC-004–005 | Completed by `CHANGELOG.md` and focused Cargo metadata. |
| DOC-006 | Completed by `docs/cheat-sheet.md`; canonical signatures remain linked rustdoc. |
| PROM-001–002 | Completed by `DEVELOPMENT.md` and `ROADMAP.md`. |
| PROM-003 | Resolved truthfully in README: no public adopters have requested listing; contributions are invited. |

## Tests and coverage

| IDs | Resolution |
|---|---|
| TEST-001 | Completed with Proptest properties for money ratios, discounting, and Gini bounds. The audit's claim that `discount_factor` decreases was corrected: the factor increases, so present value decreases. |
| TEST-002 | Completed by the cargo-fuzz `money_arithmetic` target. |
| TEST-003 | Declined: `Money` is a direct foreign type alias, so this crate cannot implement Serde's foreign traits; adding a wrapper violates the spec. |
| TEST-004 | Completed by scheduled/manual cargo-mutants CI. |
| TEST-005 | Declined as duplicate: 121 module/item doctests compile and assert their outputs directly in CI, which is stronger than output snapshots for these examples. |
| TEST-006 | Completed by the Criterion suite and benchmark audit directory. Representative category hot paths are benchmarked; trivial getters are intentionally not given noisy microbenchmarks. |
| TEST-007 | Already completed in `bin/ci`; doctests are an explicit command. |
| COV-001–003 | Completed: property, panic, boundary, absent-root, all-enum-path, and demo tests produce 100% function, line, and region coverage, enforced in CI. LLVM reports no branch counters for this source. |
| TO-001–004 | Declined: the spec mandates one comprehensive integration file per category. Splitting by testing technique would break the source-to-test map. |
| TD-001–003 | Declined: small source worked examples are clearer inline; shared fixtures/snapshots would hide inputs and duplicate doctest assertions. |

## Lint, build, and delivery

| IDs | Resolution |
|---|---|
| LINT-001–002 | Completed by RustSec and cargo-deny CI plus `deny.toml`. |
| LINT-003 | Completed by denied missing docs, denied pedantic Clippy, rustfmt, rustdoc warnings, and doctests; a second prose parser would be weaker and brittle. |
| LINT-004 | Declined: the suggested tools are not applicable Rust doc-comment formatters; rustfmt/rustdoc are authoritative. |
| LINT-005 | Completed by the code audit; all narrow allows have adjacent justification. |
| CI-001–003 | Completed by Linux/macOS/Windows matrix, nightly check, and cargo cache. |
| CI-004–005 | Completed by guarded `make release`/`bin/release` tooling and the tag-triggered publish workflow. Publishing still requires the protected environment secret. |
| CI-006 | Completed by the 100% badge, generated LCOV artifact, and Codecov upload. |
| CI-007 | Declined: this is a library with a deliberately non-primary demo; execution speed and dependency review are meaningful, demo binary size is not. |

## Code, dependency, and security audit

| IDs | Resolution |
|---|---|
| AUDIT-001–007 | Completed and recorded in `docs/audits/code-and-dependencies.md`; doctests and public API coverage are executable gates. |
| DEP-001–004 | Completed: direct dependencies are necessary, lockfile refreshed, compatible constraints retained, and automated advisory/license/source checks added. |
| SEC-001 | Completed by `SECURITY.md`. |
| SEC-002 | Completed by first-party unsafe scan and Clippy; a cargo-geiger count including dependency internals would not prove this crate's invariant more strongly. |
| SEC-003–004 | Completed: the library has no dates or I/O, documented in the audit. |

## API and error proposals

| IDs | Resolution |
|---|---|
| API-001 | Declined: explicit constructors preserve the Ratio/Percentage distinction; implicit `From<f64>` would weaken it. |
| API-002 | Declined: impossible for direct aliased `Money` under Rust's orphan rules without the wrapper forbidden by the spec. |
| API-003 | Declined: converting Ratio/Percentage back to Decimal suggests precision the `f64` formulas do not possess. |
| API-004–005 | Declined: source-shaped public structs and their one/two decidable methods are required by the spec; generic builders/validation would invent API. |
| API-006 | Declined: currency/percent parsing policy is application-specific and absent from source topics. |
| ERR-001–003 | Declined: the spec explicitly requires invariant-backed `expect`, documents real panic paths, and forbids propagating impossible currency errors. |

## Performance and observability

| IDs | Resolution |
|---|---|
| PERF-001 | Completed with repeatable Criterion profiling baselines; no bottleneck justifies code changes. |
| PERF-002 | Declined: the optimizer can inline these small functions; annotations without measurement constrain it needlessly. |
| PERF-003 | Completed by the architecture and code audit: Decimal is required for money; `f64` is retained only for reporting/transcendental formulas. |
| PERF-004 | Declined: inputs vary and calculations are cheap; global caches add synchronization and unbounded-key policy. |
| DEBUG-001 | Completed where applicable: public owned structs/enums derive Debug; foreign `Money` supplies its own implementation. |
| DEBUG-002–004 | Declined: universal Display, tracing, and dump methods are not source-domain behavior and would bloat a pure calculation library. Debug already provides inspection. |

## Additional documentation and examples

| IDs | Resolution |
|---|---|
| DOC-007–011 | Completed by mathematical-foundations, glossary, references, limitations, and performance documents. |
| EX-001–002 | Completed by `examples/module_tour.rs`, the demo, getting-started scenario, and per-item real worked examples. |
| EX-003 | Completed by `docs/spreadsheet-migration.md`. |
| EX-004 | Declined: the suggested libraries solve different domains, so a competitive table would mislead without a defined comparison corpus. |

## Promotional and community artifacts

| IDs | Resolution |
|---|---|
| TUT-001–004 | Declined as separate video/workshop/general-Rust curricula: they are outside this calculation crate; the maintained written porting and development tutorials cover contributor needs. |
| KS-001–004 | Declined: unrequested blogs, conference slides, and promotional claims need an author, audience, and publication venue; they are not product artifacts. |
| NICE-001 | Declined: Git history is the accurate contributor record and avoids duplicating personal data. |
| NICE-002–003 | Declined: no sponsor account is authorized, and committed stargazer snapshots become immediately stale. |
| NICE-004–005 | Resolved by the maintainer/contribution policy and `ROADMAP.md`; a one-maintainer library does not need invented governance roles. |
| NICE-006 | Declined: commit-message decoration is not a repository deliverable or quality control. |
| NICE-007 | Already completed by the repository's detailed `AGENTS.md`. |
| NICE-008 | Completed by checking the existing `llms.txt`/`llms.json` inventories against the 64-topic mapping and extensions. |

There are no open GitHub issues. Future work enters the backlog through a concrete issue or an
intentional spec change, rather than by silently reopening declined generic recommendations.
