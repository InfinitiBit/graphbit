# Changelog

All notable changes to the GraphBit JavaScript bindings will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial JavaScript bindings for GraphBit core library
- Complete TypeScript type definitions
- Support for all LLM providers (OpenAI, Anthropic, Ollama, Azure OpenAI)
- Workflow builder and executor
- Agent configuration and management
- Document loading and processing
- Text splitting with multiple strategies
- Embeddings generation
- JSON schema validation
- Comprehensive test suite (unit, integration, type, benchmarks)
- Full documentation and examples
- CI/CD workflows for testing and releases
- Pre-built binaries for multiple platforms

### Platform Support

- Node.js >= 16.0.0
- Linux (x64, ARM64, musl)
- macOS (x64, ARM64)
- Windows (x64)

## [1.0.0] - TBD

### Added

- First stable release
- Production-ready JavaScript bindings
- Complete API documentation
- Migration guides
- Maintenance plan

## [0.9.0] - TBD

### Added

- Beta release
- Core functionality implemented
- Basic documentation

---

## Version History

### Versioning Strategy

The JavaScript bindings follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backward-compatible functionality additions
- **PATCH** version for backward-compatible bug fixes

### Compatibility Matrix

| JS Bindings | Rust Core | Node.js | Status      |
| ----------- | --------- | ------- | ----------- |
| 1.x.x       | 0.5.x     | >=16    | In Progress |
| 0.9.x       | 0.5.x     | >=16    | Beta        |

---

## Migration Guides

For detailed migration instructions between versions, see [MIGRATION.md](./docs/MIGRATION.md).

## Contributing

See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for guidelines on contributing to this project.
