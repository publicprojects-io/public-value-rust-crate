# Security policy

## Supported versions

Security fixes target the latest published major version. Users should reproduce issues against
the newest release before reporting them.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting for this repository. Do not open a public issue for a
suspected vulnerability. Include the affected version, a minimal reproducer, impact, and any known
workaround. The maintainer aims to acknowledge a report within seven days and will coordinate a
fix and disclosure schedule with the reporter; complex reports may take longer to resolve.

This crate performs pure in-memory calculations: it contains no network access, file I/O, `unsafe`
code, or secret handling. Dependency advisories and licenses are checked in scheduled CI.
