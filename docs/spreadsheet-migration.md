# Migrating from a spreadsheet

1. Identify each workbook output and map it to the [cheat sheet](cheat-sheet.md).
2. Replace currency cells with exact `dec!` literals and `Money::from_decimal`.
3. Express percentages as `Percentage::from_percent`, not bare `42.0` values.
4. Preserve the workbook's intermediate columns as named Rust variables or result fields.
5. Turn every reconciled workbook example into a test before refactoring it.
6. Recompute source arithmetic independently; document discrepancies instead of copying errors.

Rust makes unit mix-ups and undocumented formula changes visible, but it does not validate the
policy assumptions behind a workbook. Keep input provenance and sensitivity analysis alongside the
calling application.
