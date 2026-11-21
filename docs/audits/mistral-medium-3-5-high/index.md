Here are **prioritized quality improvement tasks** for AI to execute on the `public-value` Rust crate. Organized by impact area with clear acceptance criteria. The project is currently clean (all tests pass, clippy is happy, docs build), version 1.0.2, 80 public items across 8 modules + units.

> This recommendation set has been triaged in full. See [resolution.md](resolution.md) for the
> item-by-item disposition and the repository evidence behind each completed or declined item.

---

---

## 🎯 **HIGH-IMPACT: New Visitor Onboarding & Promotion**

### **1. Project Discovery & First Impressions**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **ONB-001** | Add a "Getting Started" guide in `docs/tutorials/getting-started.md` | New visitors need a path from zero to first calculation | Guide includes: installation, first example (cost_per_outcome), module overview, where to find more info. Links from README. |
| **ONB-002** | Create a `USAGE.md` or expand README with practical examples | README has one example; needs more to show breadth | 2-3 complete examples covering different modules (e.g., SROI, HDI, cost-benefit). Each shows full imports, function calls, output. |
| **ONB-003** | Add a "Quick Examples" section to README | Front-page proof of value | 4-5 bullet-point examples with one-liners showing different calculations, linking to their modules. |
| **ONB-004** | Document the `src/main.rs` demo binary | Currently 395 bytes, minimal | Expand to demonstrate 5-6 calculations, add `--help` with usage info. |
| **ONB-005** | Create a `FAQ.md` | Reduce issue burden | Answer: "Why USD internally for GBP examples?", "Why 1.98 MSRV?", "Can I add my own formulas?", "How to contribute". |

### **2. Documentation & Promotion**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **DOC-001** | Add module-level doctests | Each module has `//!` doc but no doctest | Each `src/*.rs` module doc includes a `# Examples` section with a working doctest. |
| **DOC-002** | Generate and publish rustdoc to GitHub Pages | `docs.rs` exists but GH Pages adds redundancy | `.github/workflows/docs.yml` that builds and pushes `doc/` to `gh-pages` branch on main merge. |
| **DOC-003** | Add a "Contributing a New Topic" quick reference | `docs/tutorials/porting-a-new-topic.md` exists but could be shorter | Create a cheat-sheet version as a README in `docs/` or a section in CONTRIBUTING.md. |
| **DOC-004** | Add a `CHANGELOG.md` | No history beyond git log | Document v1.0.0, v1.0.1, v1.0.2 releases with dated entries. Link from README. |
| **DOC-005** | Improve crate metadata in `Cargo.toml` | Current keywords/categories are generic | Add specific tags: `public-sector`, `social-value`, `sroi`, `cost-benefit`. Update description to be more compelling. |
| **DOC-006** | Create a "Cheat Sheet" in `docs/cheat-sheet.md` | Quick reference for all formulas | One-page summary: every public function with its signature, one-line description, and example input/output. |
| **PROM-001** | Add a `HACKING.md` or `DEVELOPMENT.md` | Guide for setting up dev environment | Document: required tools, `make ci` workflow, how to run individual tests, debugging tips. |
| **PROM-002** | Add a `ROADMAP.md` | Signal project direction | Outline: short-term (next 6 months), long-term (next 2 years), and "not planned" items. |
| **PROM-003** | Create a "Who Uses This" section | Social proof | Add a `ADOPTERS.md` or section in README listing known users (even if just internal projects). |

---

---

## 🚀 **HIGH-IMPACT: Engineering & Code Quality**

### **1. Testing Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **TEST-001** | Add property-based tests with `proptest` | Current tests are example-based only | Add `proptest` dev-dependency. Write property tests for: `money_ratio` (a/b == b/a reciprocal), `discount_factor` (monotonic decreasing with rate), `gini_coefficient` (0 ≤ g ≤ 1). |
| **TEST-002** | Add fuzz testing for Money arithmetic | Catch edge cases in currency operations | Use `cargo-fuzz` to fuzz `Money::add`, `Money::sub`, `Money::mul` operations. |
| **TEST-003** | Create a `tests/roundtrip.rs` | Verify serialization/deserialization | Test that `Money`, `Ratio`, `Percentage` can round-trip through serialize/deserialize (if traits are added). |
| **TEST-004** | Add mutation testing | Ensure test suite quality | Use `cargo-mutants` to run mutation testing. Add to CI as a nightly job. |
| **TEST-005** | Add snapshot tests for doc examples | Prevent doc drift | Use `trycmd` to test that all doc examples compile and produce expected output. |
| **TEST-006** | Add benchmarks for all public functions | `bin/test-benchmarks` exists but empty | Use `criterion` to benchmark each module's functions. Add `benches/` directory. |
| **TEST-007** | Verify doctests in CI | Currently only run manually | Add `cargo test --doc` to `bin/ci` script (currently missing from CI). |

### **2. Code Health & Linting**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **LINT-001** | Run `cargo audit` in CI | Security vulnerability scanning | Add `cargo-audit` check to `bin/ci` and CI workflow. |
| **LINT-002** | Add `cargo-deny` for dependency licensing | Ensure license compatibility | Configure `deny.toml` to enforce license compatibility. Add check to CI. |
| **LINT-003** | Enforce consistent doc comment style | Current docs vary in structure | Add a pre-commit hook or CI check for doc comment consistency (e.g., all start with capital letter, end with period). |
| **LINT-004** | Add `cargo-typst` or `cargo-sρά` for doc formatting | Improve documentation formatting | Use a tool to format doc comments consistently. |
| **LINT-005** | Check for unused `#[allow]` annotations | Technical debt | Scan codebase for `#[allow]` and remove those that are no longer needed. |

### **3. Build & CI/CD Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **CI-001** | Add macOS and Windows CI jobs | Currently Ubuntu only | Extend CI workflow to test on macOS-latest and windows-latest. |
| **CI-002** | Add nightly Rust testing | Catch breaking changes early | Add a nightly job that runs `cargo check` with nightly Rust. |
| **CI-003** | Cache cargo index in CI | Faster CI runs | Use `actions-rs/cargo-cache` or similar for faster CI. |
| **CI-004** | Add a `make release` target | Standardize release process | Script that: bumps version, updates CHANGELOG, creates git tag, pushes to crates.io. |
| **CI-005** | Add a release workflow | Automate publishing | `.github/workflows/release.yml` that publishes to crates.io on tag push. |
| **CI-006** | Add code coverage badge to README | Visibility | Upload coverage reports to codecov.io or similar. Add badge to README. |
| **CI-007** | Add a size-check for the crate | Monitor binary size | Use `cargo-size` to track and report binary size changes. |

---

---

## 🔍 **HIGH-IMPACT: Auditing & Security**

### **1. Code Auditing**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **AUDIT-001** | Audit all `expect()` calls | Find potential panics | Review every `.expect()` in codebase. Replace with proper error handling or document why panic is acceptable. |
| **AUDIT-002** | Audit all `unwrap()` calls | Same as above | Replace or justify all `.unwrap()` calls. |
| **AUDIT-003** | Audit all unsafe code | Ensure safety | There should be no `unsafe` blocks (this is a pure calculation crate). Remove any that exist. |
| **AUDIT-004** | Check for integer overflows | Prevent undefined behavior | Audit all arithmetic operations for potential overflow. Add checks or use `checked_*` methods where appropriate. |
| **AUDIT-005** | Review all `f64` usage | Floating-point precision | Audit all `f64` operations for precision issues. Document known limitations. |
| **AUDIT-006** | Verify all doctests are accurate | Prevent misleading docs | Run all doctests and verify they match the source material's worked examples. |
| **AUDIT-007** | Check for unused public items | API surface bloat | Run `cargo deadlinks` or similar to find unused public items. Consider making private or documenting better. |

### **2. Dependency Auditing**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **DEP-001** | Audit all dependencies | Security and licensing | Review `Cargo.lock` for: outdated dependencies, security vulnerabilities, license compatibility. |
| **DEP-002** | Update all dependencies | Security patches | Run `cargo update` and test. Update to latest compatible versions. |
| **DEP-003** | Add dependency version constraints | Prevent breaking changes | Use `^` or `~` constraints appropriately in `Cargo.toml`. Document rationale. |
| **DEP-004** | Check for redundant dependencies | Bloat | Identify and remove any redundant or unused dependencies. |

### **3. Security Auditing**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **SEC-001** | Add a `SECURITY.md` | Security policy | Document: how to report security issues, response time expectations, disclosure policy. |
| **SEC-002** | Run `cargo-geiger` | Detect unsafe code | Scan for unsafe code. Should be zero for this crate. |
| **SEC-003** | Check for Y2K38 or similar date issues | Future-proofing | Audit all date/time calculations for potential overflow issues. |
| **SEC-004** | Review all file I/O | Security | This is a calculation crate; there should be no file I/O. Remove any that exist. |

---

---

## 🛠️ **MEDIUM-IMPACT: Engineering Improvements**

### **1. Type Safety & API Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **API-001** | Add `From`/`Into` implementations for units | Better ergonomics | Implement `From<f64> for Percentage`, `From<f64> for Ratio`, etc. where appropriate. |
| **API-002** | Add `Serialize`/`Deserialize` for units | Enable JSON/YAML support | Derive or implement `serde` traits for `Money`, `Ratio`, `Percentage`. |
| **API-003** | Add a `ToDecimal` trait for units | Better interop | Implement conversion from `Money`, `Ratio`, `Percentage` to `Decimal`. |
| **API-004** | Add builder patterns for complex structs | Better ergonomics | Add builder patterns for structs with many fields (e.g., `FiveCaseModel`, `StrategicTriangle`). |
| **API-005** | Add a `Validate` trait | Input validation | Implement a `validate()` method for structs that need input validation. |
| **API-006** | Add a `FromStr` implementation for units | Better parsing | Implement `FromStr` for `Money`, `Ratio`, `Percentage` to enable parsing from strings. |

### **2. Error Handling Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **ERR-001** | Create a custom error type | Better error handling | Create a `PublicValueError` enum for all error cases. Replace all `.expect()` with proper error handling. |
| **ERR-002** | Add a `Result` type alias | DRY | Create a `type Result<T> = std::result::Result<T, PublicValueError>;`. |
| **ERR-003** | Add thiserror/thiserror-img | Better error derivation | Use `thiserror` for error types. Add to dev-dependencies. |

### **3. Performance Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **PERF-001** | Profile the crate | Find bottlenecks | Use `cargo flamegraph` or similar to profile the crate. Identify and optimize bottlenecks. |
| **PERF-002** | Add `#[inline]` hints | Optimize hot paths | Add `#[inline]` to small, frequently called functions. |
| **PERF-003** | Review all `Decimal` usage | Performance | `Decimal` is slower than `f64`. Audit and justify all `Decimal` usage. |
| **PERF-004** | Use `lazy_static` or `once_cell` | Cache expensive computations | Cache expensive computations (e.g., discount factors, constants). |

### **4. Debugging & Observability**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **DEBUG-001** | Add a `Debug` derive for all public types | Better debugging | Ensure all public types derive `Debug`. |
| **DEBUG-002** | Add a `Display` implementation for all public types | Better logging | Ensure all public types implement `Display` with human-readable output. |
| **DEBUG-003** | Add a `tracing` or `log` integration | Better observability | Add optional `tracing` or `log` support for debugging. |
| **DEBUG-004** | Add a `dump()` method for complex structs | Better debugging | Add a `dump()` method for complex structs to print their state in a readable format. |

---
---

## 🧪 **MEDIUM-IMPACT: Testing & Validation**

### **1. Test Coverage Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **COV-001** | Add branch coverage | Better test quality | Use `cargo-tarpaulin` to measure and improve branch coverage. Target: 100% branch coverage. |
| **COV-002** | Add line coverage | Better test quality | Target: 100% line coverage for all public functions. |
| **COV-003** | Add edge case tests | Better robustness | Add tests for edge cases: zero values, negative values, overflow, underflow, NaN, infinity. |

### **2. Test Organization Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **TO-001** | Add a `tests/integration/` directory | Better organization | Move all integration tests to `tests/integration/`. |
| **TO-002** | Add a `tests/unit/` directory | Better organization | Move all unit tests to `tests/unit/`. |
| **TO-003** | Add a `tests/property/` directory | Better organization | Move all property-based tests to `tests/property/`. |
| **TO-004** | Add a `tests/fuzz/` directory | Better organization | Move all fuzz tests to `tests/fuzz/`. |

### **3. Test Data Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **TD-001** | Add test fixtures | Reusability | Create reusable test fixtures for common scenarios (e.g., sample `Money` values, sample `Percentage` values). |
| **TD-002** | Add a `test_utils` module | DRY | Create a `tests/test_utils.rs` module with shared test utilities. |
| **TD-003** | Add snapshot testing | Better assertions | Use `insta` or similar for snapshot testing of complex outputs. |

---
---
---

## 📚 **MEDIUM-IMPACT: Documentation & Examples**

### **1. Documentation Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **DOC-007** | Add a "Mathematical Foundations" section | Educational value | Document the mathematical theory behind each module (e.g., SROI formula, HDI calculation). |
| **DOC-008** | Add a "Glossary" section | Clarity | Create a `GLOSSARY.md` with definitions for all domain-specific terms (e.g., SROI, QALY, DALY, WELLBY). |
| **DOC-009** | Add a "References" section | Attribution | Create a `REFERENCES.md` with citations for all source material (e.g., HM Treasury Green Book, UNDP HDI technical notes). |
| **DOC-010** | Add a "Limitations" section | Honesty | Document known limitations of each calculation (e.g., assumptions, approximations, edge cases). |
| **DOC-011** | Add a "Performance Notes" section | Transparency | Document performance characteristics of each function (e.g., time complexity, space complexity). |

### **2. Example Improvements**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **EX-001** | Add a `examples/` directory | Showcase usage | Create a `examples/` directory with runnable examples for each module. |
| **EX-002** | Add a "Real-World Examples" section | Practical value | Document real-world use cases for each calculation (e.g., "Use `sroi_ratio` for social impact bonds"). |
| **EX-003** | Add a "Comparison with Spreadsheets" section | Migration guide | Document how to migrate from spreadsheet-based calculations to this crate. |
| **EX-004** | Add a "Comparison with Other Libraries" section | Competitive analysis | Compare this crate with similar libraries (e.g., Python's `pvlib`, R's `socialROI`). |

---
---
---
## 🎓 **UPSKILLING TASKS (For Human Contributors, AI can help prepare)**

### **1. Tutorial Development**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **TUT-001** | Create a "Porting a Topic" video tutorial | Onboarding | Script and storyboards for a 10-minute video walking through porting a new topic. |
| **TUT-002** | Create a "Contributing to public-value" workshop | Community building | Materials for a 1-hour workshop on contributing to the crate. |
| **TUT-003** | Create a "Rust for Data Scientists" guide | Bridge gap | Guide for data scientists new to Rust on how to use this crate. |
| **TUT-004** | Create a "Testing in Rust" guide | Upskilling | Guide for contributors on testing best practices in Rust. |

### **2. Knowledge Sharing**
| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **KS-001** | Write a blog post: "Why We Built public-value" | Promotion | Draft a blog post explaining the motivation behind the crate. |
| **KS-002** | Write a blog post: "Porting 64 Formulas from a Book to Rust" | Technical insight | Draft a blog post on the technical challenges and solutions in porting the formulas. |
| **KS-003** | Write a blog post: "Testing Strategies for a Math Library" | Best practices | Draft a blog post on testing strategies used in this crate. |
| **KS-004** | Create a talk: "Public Value Calculations in Rust" | Promotion | Prepare slides and speaker notes for a conference talk. |

---
---
---
## 🏆 **LOW-IMPACT / NICE-TO-HAVE**

| ID | Task | Why | Acceptance Criteria |
|---|---|---|---|
| **NICE-001** | Add a `CONTRIBUTORS.md` | Recognition | List all contributors with their GitHub handles. |
| **NICE-002** | Add a `FUNDING.yml` | Funding | Set up GitHub Sponsors or similar. |
| **NICE-003** | Add a `STARGAZERS.md` | Recognition | List all stargazers (if feasible). |
| **NICE-004** | Add a `GOVERNANCE.md` | Transparency | Document project governance (e.g., maintainer roles, decision-making process). |
| **NICE-005** | Add a `VISION.md` | Direction | Document the long-term vision for the project. |
| **NICE-006** | Add emoji to commit messages | Fun | Use emoji in commit messages for better readability. |
| **NICE-007** | Add a `AGENTS.md` for contributors | AI guidance | Expand the existing `AGENTS.md` with more guidance for AI contributors. |
| **NICE-008** | Add a `llms.txt` update | AI tools | Keep `llms.txt` and `llms.json` up to date with AI tools used in the project. |

---
---
---
## 🎯 **RECOMMENDED STARTING PACK (Highest ROI)**

Start with these **10 tasks** for immediate impact:

1. **ONB-001**: Getting Started guide
2. **ONB-003**: Quick Examples in README
3. **TEST-007**: Verify doctests in CI (critical gap)
4. **CI-004**: `make release` target
5. **AUDIT-001**: Audit all `expect()` calls
6. **AUDIT-006**: Verify all doctests are accurate
7. **DOC-002**: Publish rustdoc to GitHub Pages
8. **CI-001**: Add macOS and Windows CI jobs
9. **DOC-001**: Module-level doctests
10. **DOC-006**: Cheat Sheet

---
---
---
## 📊 **IMPLEMENTATION TRACKING TEMPLATE**

```markdown
# Quality Improvement Progress

## In Progress
- [ ] TASK-ID: Description (Assignee, Started)

## Completed
- [x] TASK-ID: Description (Assignee, Completed)

## Backlog
- [ ] TASK-ID: Description
```

---
---
---
## ⚠️ **CRITICAL NOTES FOR AI CONTRIBUTORS**

1. **Spec First**: Always read `spec/architecture.md` and `spec/topics.md` before making changes to `src/` or `tests/`.
2. **No Breaking Changes**: The crate is at v1.0.2. Avoid breaking changes to public APIs.
3. **Clippy Pedantic**: Every PR must pass `cargo clippy --all-targets -- -D warnings`.
4. **Doctest Everything**: Every public item must have a doctest reproducing a real worked example.
5. **MSRV Compliance**: The Minimum Supported Rust Version is 1.98. Ensure all code works with this version.
6. **Currency Invariant**: All `Money` values must be tagged with `iso::USD`, even when the doc comment cites GBP figures.
7. **No Unsafe**: This is a pure calculation crate. No `unsafe` code is permitted.
8. **License**: All contributions are licensed under MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only OR OGL-UK-3.0 OR CC-BY-NC-SA-4.0.

---
---
---
**Next Steps**: Pick a category and start with the highest-priority task. Each task has clear acceptance criteria. Would you like me to elaborate on any specific task or create a detailed implementation plan for one?
