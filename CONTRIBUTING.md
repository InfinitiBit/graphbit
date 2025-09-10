# Contributing to GraphBit

Thank you for your interest in contributing to GraphBit! This guide will help you get started with development and understand our contribution process.

### Development Setup
>[!NOTE]
>We strongly recommend that you have your own OpenAI and Anthropic API keys in order to contribute.

1. For any Linux-based distribution, use the command below to install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Now, clone the GitHub repository:
```bash
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit
```
3. Let's compile the Rust binary:
```bash
cargo build --release
```
4. Then, install Poetry and necessary packages (make sure that you have properly created a virtual environment for Python):
```bash
pip install poetry
poetry lock
poetry install
#If you do not want to install the Python dependencies for benchmark, use --with dev
poetry install --with dev
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
8. For PRs, please use the format below for branches:

- feature/yourbranchname : For feature enhancements
- doc/yourbranchname : For changes in documentation
- refactor/yourbranchname : For refactoring any codebase
- optimize/yourbranchname : For code optimization
- bugfix/yourbranchname : For bugfixes that require quick patches and do not raise critical issues
- hotfix/yourbranchname : For hotfixes that require root solutions to problems that raise critical issues

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
