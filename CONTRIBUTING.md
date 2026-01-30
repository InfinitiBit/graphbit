# Contributing to GraphBit

Thank you for your interest in contributing to GraphBit! This guide will help you get started with development and understand our contribution process.

### Fork and Clone Workflow

GraphBit follows the standard GitHub fork-based workflow for external contributors.

1. **Fork** the repository on GitHub: https://github.com/InfinitiBit/graphbit
2. **Clone your fork** locally:
```bash
git clone https://github.com/<your-github-username>/graphbit.git
cd graphbit
```
3. **Add the upstream remote** (so you can sync with the main repository):
```bash
git remote add upstream https://github.com/InfinitiBit/graphbit.git
git remote -v
```
4. **Sync your fork** before starting new work:
```bash
git fetch upstream
git checkout main
git pull origin main
git merge upstream/main
```

> Tip: If you prefer a linear history, replace `git merge upstream/main` with `git rebase upstream/main`.

### Development Setup
>[!NOTE]
>We strongly recommend that you have your own OpenAI and Anthropic API keys in order to contribute.

1. For any Linux-based distribution, use the command below to install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Now, clone the GitHub repository (preferably your fork if you are an external contributor):
```bash
git clone https://github.com/<your-github-username>/graphbit.git
cd graphbit
```
3. Let's compile the Rust binary:
```bash
cargo build --release
```
4. Install Python dependencies using either Poetry or uv (both are fully supported):

**Option A: Using Poetry**
```bash
poetry install
# If you do not want to install the Python dependencies for benchmark, use --with dev
poetry install --with dev
```

**Option B: Using uv (faster alternative)**
```bash
# Sync all dependencies (creates .venv automatically)
uv sync

# Or sync without benchmark dependencies
uv sync --extra dev
```
5. Now, let's create the Python release version:
```bash
cd python/
maturin develop --release
```
6. If you want to contribute to this package, please install the pre-commit hook:
```bash
pre-commit clean
pre-commit install
pre-commit run --all-files
```
Before you run the pre-commit hook, make sure you have put the relevant keys in your bash/zsh file:
```bash
export OPENAI_API_KEY=your_openai_api_key
export ANTHROPIC_API_KEY=your_anthropic_api_key
```
7. After making changes to the code, please run the integration tests for both Python and Rust:
```bash
cargo test --workspace
pytest .
```
8. For PRs, please use the branch naming conventions below.

**Best practices**
- Use `kebab-case` and keep names concise but descriptive (avoid `feature/fix-stuff`)
- Keep one branch focused on one change set
- If you use issue/ticket IDs, you may include them (e.g., `bugfix/123-fix-null-pointer`)

**Branch prefixes (with examples)**
- `feature/<short-description>`: For feature enhancements
  - Example: `feature/version-management`
- `doc/<short-description>`: For changes in documentation
  - Example: `doc/code-of-conduct`
- `refactor/<short-description>`: For refactoring any codebase
  - Example: `refactor/extract-workflow-runner`
- `optimize/<short-description>`: For code optimization / performance improvements
  - Example: `optimize/faster-vector-search`
- `bugfix/<short-description>`: For bugfixes that require quick patches and do not raise critical issues
  - Example: `bugfix/fix-cli-config-parse`
- `hotfix/<short-description>`: For hotfixes that require root solutions to problems that raise critical issues
  - Example: `hotfix/restore-release-workflow`

> Note: This repository also uses `enhance/<...>` and `infra/<...>` branch types in `scripts/branch-config.json` for non-breaking improvements and infrastructure/tooling work.

### Commit Message Style

GraphBit uses **Conventional Commits**.

**Format**

`type(scope): short description`

**Common types**
- `feat`: new functionality
- `fix`: bug fixes
- `docs`: documentation-only changes
- `refactor`: refactoring without behavior change
- `style`: formatting-only changes (no functional changes)
- `test`: adding/updating tests
- `chore`: tooling/maintenance/dependency updates
- `ci`: CI/CD changes
- `perf`: performance improvements

**Guidelines**
- Use the imperative mood ("add", "fix", "update")
- Keep the subject line short and specific
- Use a scope when it helps reviewers (e.g., `docs(security): ...`, `feat(core): ...`)

**Examples**
- Good: `feat(core): add workflow retry policy`
- Good: `docs: add Code of Conduct (Contributor Covenant)`
- Bad: `updates`
- Bad: `Fixed things`

> Note: This repo includes a `commitizen` pre-commit hook that validates commit messages.

### Pull Request (PR) Checklist

Before submitting a PR, please ensure:

- [ ] Branch is up-to-date with `main` (or your target branch)
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
- [ ] Documentation is updated if the change affects public behavior or APIs
- [ ] `CHANGELOG.md` is updated if applicable
- [ ] PR description clearly explains **what** changed, **why**, and any relevant context

### Code Review Expectations

- Maintainers aim to respond to PRs as soon as possible; for most PRs, expect an initial review within **a few business days**.
- Keep PRs focused and provide enough context for reviewers (screenshots/logs where helpful).
- Address review comments by pushing follow-up commits and replying to threads with rationale.
- PRs typically require maintainer approval before merge.
- Merge permissions are held by GraphBit maintainers (InfinitiBit GmbH).

This framework utilizes the following code quality tools:

**Rust:**
- clippy: Code linting
- fmt: Code formatting  
- cargo-audit: Security audits

**Python:**
- flake8: Code linting
- black: Code formatting
- isort: Import sorting
- mypy: Type checking
- bandit: Security checks

**Additional Tools:**
- typos: Spell checking
- hadolint: Dockerfile linting
- shellcheck: Shell script linting 
