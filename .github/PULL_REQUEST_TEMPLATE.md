## Description

<!--
Explain WHAT changed and WHY. Include context, trade-offs, and any user-facing impact.
Link to relevant docs or design notes.
-->

### What changed?
-

### Why?
-

## Related issues / tickets

<!-- Use GitHub keywords so issues auto-close when merged. Examples: Fixes #123, Closes #123 -->
Fixes #

## Type of change

<!-- Check all that apply -->
- [ ] fix (bug fix)
- [ ] feat (new feature)
- [ ] refactor (no behavior change)
- [ ] perf (performance improvement)
- [ ] docs (documentation-only)
- [ ] style (formatting-only)
- [ ] test (adds/updates tests)
- [ ] chore (tooling/maintenance/dependency update)
- [ ] ci (CI/CD changes)
- [ ] breaking change
- [ ] security

## How to test / reproduce

<!-- Provide steps for reviewers to validate the change locally. -->
1.
2.
3.

## Checklist (required)

<!-- See CONTRIBUTING.md for full contributor guidance. -->
- [ ] Branch is up-to-date with `main` (or the target branch)
- [ ] Tests pass:
  - [ ] `cargo test --workspace`
  - [ ] `pytest .`
- [ ] Pre-commit hooks pass:
  - [ ] `pre-commit run --all-files`
- [ ] Formatting is applied:
  - [ ] Rust: `cargo fmt`
  - [ ] Python: `black` and `isort` (or run via pre-commit)
- [ ] Linting/type checks pass:
  - [ ] Rust: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - [ ] Python: `flake8` and `mypy` (or run via pre-commit)
- [ ] Documentation updated if public behavior/APIs changed
- [ ] `CHANGELOG.md` updated if applicable

## Notes for reviewers

<!--
Optional: call out areas that need extra attention, follow-ups, or risk.
Include screenshots/logs if relevant.
-->
-

