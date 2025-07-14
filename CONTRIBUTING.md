# Contributing to GraphBit

Thank you for your interest in contributing to GraphBit! This guide will help you get started with development and understand our contribution process.

### Development Setup
>[!NOTE]
>We strongly recommend that you have your own OPENAI and ANTHROPIC api keys in order to contribute.

1. For any linux-based distribution, use the below command to install rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Now, clone the github repository:
```bash
git clone https://github.com/InfinitiBit/graphbit.git
cd graphbit
```
3. Lets compile the rust binary:
```bash
cargo build --release
```
4. Then, install the poetry and necessary packages (make sure that you have properly created a virtual environment for python):
```bash
pip install poetry
poetry lock
poetry install
#If you do not want to install the python depenendices for benchmark, use --with dev
poetry install --with dev
```
5. Now, lets create the python release version:
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
Before you run the pre-commit hook, make sure you have put the relevant keys in your bash/zsh file.
```bash
export OPENAI_API_KEY=your openai api key
export ANTHROPIC_API_KEY=your anthropic api key
export HUGGINGFACE_API_KEY=your huggingface api key
```
7. After your changes in the code, please run the integration tests for both python and rust:
```bash
cargo test --workspace
pytest .
```
8. For PR, please use the below format for branches:

feature/yourbranchname : For feature enhancement
doc/yourbranchname : For changes in documentation
refactor/yourbranchname : For refactoring any code base
optimize/yourbranchname : For code optimization
bugfix/yourbranchname : For bugfix that requires quick patch and does not raise criticial issue.
hotfix/yourbranchname : For hotfix that requires root solution of a problem that raises criticial issue.

