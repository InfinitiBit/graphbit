# Changelog

## [0.4.0] - 2025-09-08

### ✨ New Features

- Test rust : add python binding tests, helper functions by @masbulhaider in [#107](https://github.com/InfinitiBit/graphbit/pull/107) ([63b19d1](https://github.com/InfinitiBit/graphbit/commit/63b19d16)) on 2025-09-03
- Added tool calling support by @yeahia-sarker in [#131](https://github.com/InfinitiBit/graphbit/pull/131) ([909e987](https://github.com/InfinitiBit/graphbit/commit/909e9877)) on 2025-08-28
- Pr review addressed by @tanima-hossain ([f34812e](https://github.com/InfinitiBit/graphbit/commit/f34812e6)) on 2025-08-20
- Review addressed by @tanima-hossain ([ec14e6e](https://github.com/InfinitiBit/graphbit/commit/ec14e6e6)) on 2025-08-18
- Updated python-api.md by @junaidhossain04 ([31d865d](https://github.com/InfinitiBit/graphbit/commit/31d865df)) on 2025-08-15
- Updated configuration.md by @junaidhossain04 ([5708d18](https://github.com/InfinitiBit/graphbit/commit/5708d181)) on 2025-08-15
- Refactored and updated python-api.md file. by @junaidhossain04 ([9bf2b30](https://github.com/InfinitiBit/graphbit/commit/9bf2b302)) on 2025-08-15
- Update astradb_integration.md by @masbulhaider ([e58c156](https://github.com/InfinitiBit/graphbit/commit/e58c1567)) on 2025-08-14
- Added nodejs binding by @yeahia-sarker in [#97](https://github.com/InfinitiBit/graphbit/pull/97) ([a1084d0](https://github.com/InfinitiBit/graphbit/commit/a1084d01)) on 2025-08-14
- : Rust Core, Rust Python Binding: Agentic Workflow with Dep-Batching and Parent Preamble #87 by @jaid-jashim in [#87](https://github.com/InfinitiBit/graphbit/pull/87) ([11b01b2](https://github.com/InfinitiBit/graphbit/commit/11b01b2c)) on 2025-08-10
- Updated the readme file in python folder. by @junaidhossain04 ([9125fa7](https://github.com/InfinitiBit/graphbit/commit/9125fa70)) on 2025-08-08
- Updated readme by @tanima-hossain ([5c5dee7](https://github.com/InfinitiBit/graphbit/commit/5c5dee71)) on 2025-08-08
- Added document loader support by @yeahia-sarker in [#84](https://github.com/InfinitiBit/graphbit/pull/84) ([b80f3cd](https://github.com/InfinitiBit/graphbit/commit/b80f3cd8)) on 2025-08-06
- Added SECURITY.md by @yeahia-sarker in [#68](https://github.com/InfinitiBit/graphbit/pull/68) ([715459b](https://github.com/InfinitiBit/graphbit/commit/715459bf)) on 2025-07-27
- Added perplexity support by @yeahia-sarker in [#67](https://github.com/InfinitiBit/graphbit/pull/67) ([3ee41b7](https://github.com/InfinitiBit/graphbit/commit/3ee41b7b)) on 2025-07-27
- Added deepseek support by @yeahia-sarker in [#66](https://github.com/InfinitiBit/graphbit/pull/66) ([538e152](https://github.com/InfinitiBit/graphbit/commit/538e1521)) on 2025-07-27
- Implement LLM-Graphbit-Playwright Browser Automation Agent by @jaid-jashim in [#52](https://github.com/InfinitiBit/graphbit/pull/52) ([ee970fb](https://github.com/InfinitiBit/graphbit/commit/ee970fba)) on 2025-07-26
- Added textsplitter by @yeahia-sarker in [#64](https://github.com/InfinitiBit/graphbit/pull/64) ([7bf6a7d](https://github.com/InfinitiBit/graphbit/commit/7bf6a7d8)) on 2025-07-26
- Added chatbot development example by @tanimahossain-infiniti in [#38](https://github.com/InfinitiBit/graphbit/pull/38) ([e605855](https://github.com/InfinitiBit/graphbit/commit/e6058554)) on 2025-07-21
- Integrated google search api with graphbit by @junaid-hossain in [#47](https://github.com/InfinitiBit/graphbit/pull/47) ([915eabc](https://github.com/InfinitiBit/graphbit/commit/915eabc0)) on 2025-07-21

### 🐛 Bug Fixes

- Infinite loop issue from character splitter by @tanimahossain-infiniti in [#151](https://github.com/InfinitiBit/graphbit/pull/151) ([936f549](https://github.com/InfinitiBit/graphbit/commit/936f5494)) on 2025-09-05
- Sentence splitter issue by @tanimahossain-infiniti in [#148](https://github.com/InfinitiBit/graphbit/pull/148) ([fca76f9](https://github.com/InfinitiBit/graphbit/commit/fca76f9c)) on 2025-09-05
- Refactored and fixed validation md by @tanima-hossain ([ad38de7](https://github.com/InfinitiBit/graphbit/commit/ad38de7e)) on 2025-08-20
- Anthropic llm config issue fixed by @tanimahossain-infiniti in [#110](https://github.com/InfinitiBit/graphbit/pull/110) ([0d0e0b1](https://github.com/InfinitiBit/graphbit/commit/0d0e0b1e)) on 2025-08-18
- Connection edge cases, unique IDs, and cyclic workflow checks by @jaid-jashim in [#102](https://github.com/InfinitiBit/graphbit/pull/102) ([f9bb183](https://github.com/InfinitiBit/graphbit/commit/f9bb1835)) on 2025-08-15
- Title error fix. by @junaidhossain04 ([71b2935](https://github.com/InfinitiBit/graphbit/commit/71b29359)) on 2025-08-14
- Make GraphBit benchmark CPU affinity logic cross-platform (macOS support) by @jj-devhub in [#51](https://github.com/InfinitiBit/graphbit/pull/51) ([f7d97f7](https://github.com/InfinitiBit/graphbit/commit/f7d97f7e)) on 2025-07-21
- Add macOS fallback for sched_getaffinity by @jj-devhub in [#50](https://github.com/InfinitiBit/graphbit/pull/50) ([11dbbae](https://github.com/InfinitiBit/graphbit/commit/11dbbae2)) on 2025-07-21
- Updated black and dependency check by @yeahia-sarker in [#48](https://github.com/InfinitiBit/graphbit/pull/48) ([7c62b2b](https://github.com/InfinitiBit/graphbit/commit/7c62b2bd)) on 2025-07-21
- **pre-commit**: Resolve hook failures and improve security compliance by @jj-devhub in [#45](https://github.com/InfinitiBit/graphbit/pull/45) ([b8e6294](https://github.com/InfinitiBit/graphbit/commit/b8e6294b)) on 2025-07-17

### ⚡ Performance

- Implement comprehensive Cargo.toml optimization with seamless Maturin integration - Add unified workspace dependency management across all crates - Implement multiple build profiles (dev, release, release-python) with size optimizations - Add cross-platform compatibility with Windows/Unix-specific configurations - Integrate enterprise-grade linting with 103+ quality rules - Optimize Python bindings with Maturin for 25% smaller wheels - Achieve 50% faster builds through workspace dependency caching - Add production-ready Python wheel generation (2.93MB optimized) - Maintain full backward compatibility for all existing Python development workflows by @jj-devhub ([c122ba9](https://github.com/InfinitiBit/graphbit/commit/c122ba9c)) on 2025-09-08
- Performance.md validated by @tanimahossain-infiniti in [#128](https://github.com/InfinitiBit/graphbit/pull/128) ([5734fb9](https://github.com/InfinitiBit/graphbit/commit/5734fb9e)) on 2025-08-25
- AI LLM Multi-Agent Framework Benchmark Comparison Performance Report Summary Across Intel and AMD Virtual Machines by @jaid-jashim in [#65](https://github.com/InfinitiBit/graphbit/pull/65) ([aabaa6a](https://github.com/InfinitiBit/graphbit/commit/aabaa6ae)) on 2025-07-27

### 🗑️ Removed

- Refactor] removed unnecessary packages from pyproject toml by @jaid-jashim in [#116](https://github.com/InfinitiBit/graphbit/pull/116) ([6921160](https://github.com/InfinitiBit/graphbit/commit/69211603)) on 2025-08-19
- Removed Node.condition by @tanima-hossain ([0c0f6f1](https://github.com/InfinitiBit/graphbit/commit/0c0f6f1d)) on 2025-08-18
- Removed doc site and other relevant files and codes by @jaid-jashim in [#101](https://github.com/InfinitiBit/graphbit/pull/101) ([a8c3dc8](https://github.com/InfinitiBit/graphbit/commit/a8c3dc8b)) on 2025-08-14
- Delete Astra DB test integration by @masbul-haider-ovi ([c4acc0f](https://github.com/InfinitiBit/graphbit/commit/c4acc0f7)) on 2025-08-11

### 📚 Documentation

- Root readme architecture diagram by @junaid-hossain in [#140](https://github.com/InfinitiBit/graphbit/pull/140) ([ead004c](https://github.com/InfinitiBit/graphbit/commit/ead004c7)) on 2025-09-03
- About file added by @tanimahossain-infiniti in [#89](https://github.com/InfinitiBit/graphbit/pull/89) ([013cbf4](https://github.com/InfinitiBit/graphbit/commit/013cbf44)) on 2025-09-03
- Framework benchmark report updated. by @junaid-hossain in [#134](https://github.com/InfinitiBit/graphbit/pull/134) ([5dbaa8e](https://github.com/InfinitiBit/graphbit/commit/5dbaa8ee)) on 2025-09-02
- Benchmark readme update. by @junaid-hossain in [#135](https://github.com/InfinitiBit/graphbit/pull/135) ([bb615cf](https://github.com/InfinitiBit/graphbit/commit/bb615cf8)) on 2025-09-02
- Updated root readme by @junaid-hossain in [#132](https://github.com/InfinitiBit/graphbit/pull/132) ([d184769](https://github.com/InfinitiBit/graphbit/commit/d184769b)) on 2025-09-02
- Validated reliability.md file by @tanimahossain-infiniti in [#122](https://github.com/InfinitiBit/graphbit/pull/122) ([5030fe0](https://github.com/InfinitiBit/graphbit/commit/5030fe0d)) on 2025-08-25
- Document loaders documentation. by @junaid-hossain in [#130](https://github.com/InfinitiBit/graphbit/pull/130) ([c96b9d6](https://github.com/InfinitiBit/graphbit/commit/c96b9d6f)) on 2025-08-25
- Agents.md validated by @junaid-hossain in [#111](https://github.com/InfinitiBit/graphbit/pull/111) ([d0770bc](https://github.com/InfinitiBit/graphbit/commit/d0770bc3)) on 2025-08-25
- Monitoring.md by @junaid-hossain in [#124](https://github.com/InfinitiBit/graphbit/pull/124) ([dbd4cdc](https://github.com/InfinitiBit/graphbit/commit/dbd4cdcd)) on 2025-08-22
- Concepts.md, embeddings.md and async-vs-sync.md. by @junaid-hossain in [#114](https://github.com/InfinitiBit/graphbit/pull/114) ([c52873c](https://github.com/InfinitiBit/graphbit/commit/c52873cc)) on 2025-08-22
- GCP integration with Graphbit. by @junaid-hossain in [#70](https://github.com/InfinitiBit/graphbit/pull/70) ([a77ea7f](https://github.com/InfinitiBit/graphbit/commit/a77ea7fb)) on 2025-08-21
- Update CHANGELOG.md by @tanimahossain-infiniti in [#96](https://github.com/InfinitiBit/graphbit/pull/96) ([72b81f7](https://github.com/InfinitiBit/graphbit/commit/72b81f75)) on 2025-08-19
- Updated root readme file. by @junaid-hossain in [#57](https://github.com/InfinitiBit/graphbit/pull/57) ([e67732f](https://github.com/InfinitiBit/graphbit/commit/e67732f6)) on 2025-08-18
- Added a updated version copy of ai framework comparison report by @jaid-jashim in [#72](https://github.com/InfinitiBit/graphbit/pull/72) ([2e2c098](https://github.com/InfinitiBit/graphbit/commit/2e2c0981)) on 2025-08-14
- Updated Documentation of Azure integration with Graphbit. by @junaidhossain04 ([c72660b](https://github.com/InfinitiBit/graphbit/commit/c72660b4)) on 2025-08-14
- Update AstraDB documentation by @masbul-haider-ovi ([4d98dac](https://github.com/InfinitiBit/graphbit/commit/4d98dace)) on 2025-08-14
- Elasticsearch Integration [Connector] by @tanimahossain-infiniti in [#77](https://github.com/InfinitiBit/graphbit/pull/77) ([350cb3b](https://github.com/InfinitiBit/graphbit/commit/350cb3bc)) on 2025-08-08
- IBM Db2 integration with graphbit. by @junaid-hossain in [#76](https://github.com/InfinitiBit/graphbit/pull/76) ([5030633](https://github.com/InfinitiBit/graphbit/commit/50306331)) on 2025-08-08
- Add complete MkDocs documentation site using Material theme by @jaid-jashim in [#79](https://github.com/InfinitiBit/graphbit/pull/79) ([8830fec](https://github.com/InfinitiBit/graphbit/commit/8830fec8)) on 2025-07-31
- Weaviate Integration with Graphbit by @tanimahossain-infiniti in [#75](https://github.com/InfinitiBit/graphbit/pull/75) ([97f67fb](https://github.com/InfinitiBit/graphbit/commit/97f67fbc)) on 2025-07-29
- Milvus Integration with Graphbit by @tanimahossain-infiniti in [#74](https://github.com/InfinitiBit/graphbit/pull/74) ([607c7da](https://github.com/InfinitiBit/graphbit/commit/607c7dae)) on 2025-07-29
- FAISS Integration by @tanimahossain-infiniti in [#60](https://github.com/InfinitiBit/graphbit/pull/60) ([31aa17a](https://github.com/InfinitiBit/graphbit/commit/31aa17a1)) on 2025-07-29
- Documentation: Azure integration with GraphBit. by @junaidhossain04 ([c29fa55](https://github.com/InfinitiBit/graphbit/commit/c29fa55b)) on 2025-07-29
- AWS boto3 integration with Graphbit. by @junaid-hossain in [#63](https://github.com/InfinitiBit/graphbit/pull/63) ([e1dc8bf](https://github.com/InfinitiBit/graphbit/commit/e1dc8bf2)) on 2025-07-27
- Created async-vs-sync.md file. by @junaid-hossain in [#62](https://github.com/InfinitiBit/graphbit/pull/62) ([c8676c0](https://github.com/InfinitiBit/graphbit/commit/c8676c08)) on 2025-07-26
- Chromadb Integration with GraphBit by @tanimahossain-infiniti in [#59](https://github.com/InfinitiBit/graphbit/pull/59) ([8aaf323](https://github.com/InfinitiBit/graphbit/commit/8aaf3231)) on 2025-07-26
- Mariadb basic integration with GraphBit by @tanimahossain-infiniti in [#56](https://github.com/InfinitiBit/graphbit/pull/56) ([1de6cc9](https://github.com/InfinitiBit/graphbit/commit/1de6cc90)) on 2025-07-23
- Updated embeddings documentation file by @junaid-hossain in [#58](https://github.com/InfinitiBit/graphbit/pull/58) ([00920b6](https://github.com/InfinitiBit/graphbit/commit/00920b6c)) on 2025-07-23
- Added pgvector documentation as connector by @junaid-hossain in [#55](https://github.com/InfinitiBit/graphbit/pull/55) ([d45f6e2](https://github.com/InfinitiBit/graphbit/commit/d45f6e2c)) on 2025-07-23
- Added qdrant integration documentation by @tanimahossain-infiniti in [#54](https://github.com/InfinitiBit/graphbit/pull/54) ([0061028](https://github.com/InfinitiBit/graphbit/commit/00610281)) on 2025-07-22
- Added pinecone integration documentation by @tanimahossain-infiniti in [#44](https://github.com/InfinitiBit/graphbit/pull/44) ([3e4a657](https://github.com/InfinitiBit/graphbit/commit/3e4a6575)) on 2025-07-22
- MongoDB integration with graphbit by @junaid-hossain in [#53](https://github.com/InfinitiBit/graphbit/pull/53) ([238cae2](https://github.com/InfinitiBit/graphbit/commit/238cae2a)) on 2025-07-22

### 🧪 Testing

- Fix unit test bugs by @masbulhaider in [#145](https://github.com/InfinitiBit/graphbit/pull/145) ([82a2bbf](https://github.com/InfinitiBit/graphbit/commit/82a2bbfa)) on 2025-09-05
- Integration tests for doc load, workflow and error by @masbulhaider in [#123](https://github.com/InfinitiBit/graphbit/pull/123) ([daf5328](https://github.com/InfinitiBit/graphbit/commit/daf53285)) on 2025-09-03
- Unit test for types, error in rust by @masbulhaider in [#125](https://github.com/InfinitiBit/graphbit/pull/125) ([ed37712](https://github.com/InfinitiBit/graphbit/commit/ed37712b)) on 2025-09-03
- Unit tests for document loader & serialization by @masbulhaider in [#127](https://github.com/InfinitiBit/graphbit/pull/127) ([e32bec0](https://github.com/InfinitiBit/graphbit/commit/e32bec0e)) on 2025-09-03
- Test rust adds extended unit tests for agent, concurrency & graph by @masbulhaider in [#126](https://github.com/InfinitiBit/graphbit/pull/126) ([614bd1f](https://github.com/InfinitiBit/graphbit/commit/614bd1f2)) on 2025-09-03
- Test rust : add document loader, embedding, text splitter unit tests by @masbulhaider in [#106](https://github.com/InfinitiBit/graphbit/pull/106) ([ea87a03](https://github.com/InfinitiBit/graphbit/commit/ea87a030)) on 2025-09-03
- Tests rust: add type, validation & error unit tests by @masbulhaider in [#105](https://github.com/InfinitiBit/graphbit/pull/105) ([f00fdc1](https://github.com/InfinitiBit/graphbit/commit/f00fdc11)) on 2025-09-03
- LLM, agent, workflow tests; refactored validation & workflow integration by @masbulhaider in [#104](https://github.com/InfinitiBit/graphbit/pull/104) ([b3b5a55](https://github.com/InfinitiBit/graphbit/commit/b3b5a55e)) on 2025-09-03
- **py**: Add unit tests for import, init, security & workflow failures by @masbulhaider in [#117](https://github.com/InfinitiBit/graphbit/pull/117) ([c609c43](https://github.com/InfinitiBit/graphbit/commit/c609c431)) on 2025-08-31
- Unit tests for edge cases, client & configuration failure by @masbulhaider in [#113](https://github.com/InfinitiBit/graphbit/pull/113) ([3f97684](https://github.com/InfinitiBit/graphbit/commit/3f976848)) on 2025-08-31
- Added basic unit testing by @masbulhaider in [#95](https://github.com/InfinitiBit/graphbit/pull/95) ([b60003c](https://github.com/InfinitiBit/graphbit/commit/b60003cf)) on 2025-08-31
- Unit tests for doc processing, text splitting, embedding by @masbulhaider in [#94](https://github.com/InfinitiBit/graphbit/pull/94) ([c1fb531](https://github.com/InfinitiBit/graphbit/commit/c1fb5313)) on 2025-08-31
- Unit tests LLM clients coverage; align workflow connect tests by @masbulhaider in [#93](https://github.com/InfinitiBit/graphbit/pull/93) ([32513a2](https://github.com/InfinitiBit/graphbit/commit/32513a24)) on 2025-08-31
- Python tests : add and improve LLM, executor, and document loader integration tests by @masbulhaider in [#90](https://github.com/InfinitiBit/graphbit/pull/90) ([95f6343](https://github.com/InfinitiBit/graphbit/commit/95f6343c)) on 2025-08-14
- Add Astra DB integration test file by @masbul-haider-ovi ([a53bb7e](https://github.com/InfinitiBit/graphbit/commit/a53bb7ec)) on 2025-08-11

### 🔧 Maintenance

- All the benchmark scipts file. by @junaid-hossain in [#133](https://github.com/InfinitiBit/graphbit/pull/133) ([7076cde](https://github.com/InfinitiBit/graphbit/commit/7076cde5)) on 2025-08-30
- Getting-started folder. by @junaid-hossain in [#108](https://github.com/InfinitiBit/graphbit/pull/108) ([f220d27](https://github.com/InfinitiBit/graphbit/commit/f220d274)) on 2025-08-25
- Refactored and validated. by @junaid-hossain in [#121](https://github.com/InfinitiBit/graphbit/pull/121) ([dc44712](https://github.com/InfinitiBit/graphbit/commit/dc44712e)) on 2025-08-22
- Refactored import graphbit in benchmark by @tanimahossain-infiniti in [#100](https://github.com/InfinitiBit/graphbit/pull/100) ([bece4b8](https://github.com/InfinitiBit/graphbit/commit/bece4b82)) on 2025-08-22
- Validated readme and refactored by @tanima-hossain ([207c869](https://github.com/InfinitiBit/graphbit/commit/207c8699)) on 2025-08-19
- Refactored executors examples readme by @tanima-hossain ([355c8b4](https://github.com/InfinitiBit/graphbit/commit/355c8b4e)) on 2025-08-19
- Refactored executors and connectors format by @tanima-hossain ([8e48a38](https://github.com/InfinitiBit/graphbit/commit/8e48a38e)) on 2025-08-19
- Refactored node-types.md file. by @junaidhossain04 ([985bd32](https://github.com/InfinitiBit/graphbit/commit/985bd328)) on 2025-08-15
- Refactored graphbit import by @tanima-hossain ([3f73989](https://github.com/InfinitiBit/graphbit/commit/3f73989d)) on 2025-08-14
- Refactored configuration.md file. by @junaidhossain04 ([beec6f0](https://github.com/InfinitiBit/graphbit/commit/beec6f00)) on 2025-08-14
- Formatted the codebase for clippy compatibility. by @junaid-hossain in [#49](https://github.com/InfinitiBit/graphbit/pull/49) ([4a26a80](https://github.com/InfinitiBit/graphbit/commit/4a26a804)) on 2025-07-21

### 🔄 Others

- Dynamics-graph.md by @junaid-hossain in [#119](https://github.com/InfinitiBit/graphbit/pull/119) ([6a00ec3](https://github.com/InfinitiBit/graphbit/commit/6a00ec31)) on 2025-08-22
- Workflow builder readme validated by @tanimahossain-infiniti in [#112](https://github.com/InfinitiBit/graphbit/pull/112) ([d721034](https://github.com/InfinitiBit/graphbit/commit/d721034b)) on 2025-08-22
- Slight changes made. by @junaidhossain04 ([a420e33](https://github.com/InfinitiBit/graphbit/commit/a420e33d)) on 2025-08-19
- Graphbit init call auto by default by @jaid-jashim in [#115](https://github.com/InfinitiBit/graphbit/pull/115) ([62318f4](https://github.com/InfinitiBit/graphbit/commit/62318f42)) on 2025-08-19
- Docs examples folder read me checked by @tanima-hossain ([ca6377c](https://github.com/InfinitiBit/graphbit/commit/ca6377c7)) on 2025-08-18
- Removal of the script. by @junaidhossain04 ([fd0f905](https://github.com/InfinitiBit/graphbit/commit/fd0f9058)) on 2025-08-14
- Typo in codes by @tanima-hossain ([e47662f](https://github.com/InfinitiBit/graphbit/commit/e47662fc)) on 2025-08-13
- Changed title by @tanima-hossain ([ae9fa0d](https://github.com/InfinitiBit/graphbit/commit/ae9fa0d3)) on 2025-08-13
- Tutorial: AstraDB integration with graphbit. by @masbul-haider-ovi ([5766d32](https://github.com/InfinitiBit/graphbit/commit/5766d325)) on 2025-08-11
- Feature/docs site by @jaid-jashim in [#83](https://github.com/InfinitiBit/graphbit/pull/83) ([a5074b9](https://github.com/InfinitiBit/graphbit/commit/a5074b96)) on 2025-08-09

---
**Total Changes**: 105
**Changes by Category**: ✨ New Features: 20 | 🐛 Bug Fixes: 10 | ⚡ Performance: 3 | 🗑️ Removed: 4 | 📚 Documentation: 32 | 🧪 Testing: 15 | 🔧 Maintenance: 11 | 🔄 Others: 10

## [0.4.1] - 2025-09-08

### ✨ New Features

- Test rust : add python binding tests, helper functions by @masbulhaider in [#107](https://github.com/InfinitiBit/graphbit/pull/107) ([63b19d1](https://github.com/InfinitiBit/graphbit/commit/63b19d16)) on 2025-09-03
- Added tool calling support by @yeahia-sarker in [#131](https://github.com/InfinitiBit/graphbit/pull/131) ([909e987](https://github.com/InfinitiBit/graphbit/commit/909e9877)) on 2025-08-28
- Pr review addressed by @tanima-hossain ([f34812e](https://github.com/InfinitiBit/graphbit/commit/f34812e6)) on 2025-08-20
- Review addressed by @tanima-hossain ([ec14e6e](https://github.com/InfinitiBit/graphbit/commit/ec14e6e6)) on 2025-08-18
- Updated python-api.md by @junaidhossain04 ([31d865d](https://github.com/InfinitiBit/graphbit/commit/31d865df)) on 2025-08-15
- Updated configuration.md by @junaidhossain04 ([5708d18](https://github.com/InfinitiBit/graphbit/commit/5708d181)) on 2025-08-15
- Refactored and updated python-api.md file. by @junaidhossain04 ([9bf2b30](https://github.com/InfinitiBit/graphbit/commit/9bf2b302)) on 2025-08-15
- Update astradb_integration.md by @masbulhaider ([e58c156](https://github.com/InfinitiBit/graphbit/commit/e58c1567)) on 2025-08-14
- Added nodejs binding by @yeahia-sarker in [#97](https://github.com/InfinitiBit/graphbit/pull/97) ([a1084d0](https://github.com/InfinitiBit/graphbit/commit/a1084d01)) on 2025-08-14
- : Rust Core, Rust Python Binding: Agentic Workflow with Dep-Batching and Parent Preamble #87 by @jaid-jashim in [#87](https://github.com/InfinitiBit/graphbit/pull/87) ([11b01b2](https://github.com/InfinitiBit/graphbit/commit/11b01b2c)) on 2025-08-10
- Updated the readme file in python folder. by @junaidhossain04 ([9125fa7](https://github.com/InfinitiBit/graphbit/commit/9125fa70)) on 2025-08-08
- Updated readme by @tanima-hossain ([5c5dee7](https://github.com/InfinitiBit/graphbit/commit/5c5dee71)) on 2025-08-08
- Added document loader support by @yeahia-sarker in [#84](https://github.com/InfinitiBit/graphbit/pull/84) ([b80f3cd](https://github.com/InfinitiBit/graphbit/commit/b80f3cd8)) on 2025-08-06
- Added SECURITY.md by @yeahia-sarker in [#68](https://github.com/InfinitiBit/graphbit/pull/68) ([715459b](https://github.com/InfinitiBit/graphbit/commit/715459bf)) on 2025-07-27
- Added perplexity support by @yeahia-sarker in [#67](https://github.com/InfinitiBit/graphbit/pull/67) ([3ee41b7](https://github.com/InfinitiBit/graphbit/commit/3ee41b7b)) on 2025-07-27
- Added deepseek support by @yeahia-sarker in [#66](https://github.com/InfinitiBit/graphbit/pull/66) ([538e152](https://github.com/InfinitiBit/graphbit/commit/538e1521)) on 2025-07-27
- Implement LLM-Graphbit-Playwright Browser Automation Agent by @jaid-jashim in [#52](https://github.com/InfinitiBit/graphbit/pull/52) ([ee970fb](https://github.com/InfinitiBit/graphbit/commit/ee970fba)) on 2025-07-26
- Added textsplitter by @yeahia-sarker in [#64](https://github.com/InfinitiBit/graphbit/pull/64) ([7bf6a7d](https://github.com/InfinitiBit/graphbit/commit/7bf6a7d8)) on 2025-07-26
- Added chatbot development example by @tanimahossain-infiniti in [#38](https://github.com/InfinitiBit/graphbit/pull/38) ([e605855](https://github.com/InfinitiBit/graphbit/commit/e6058554)) on 2025-07-21
- Integrated google search api with graphbit by @junaid-hossain in [#47](https://github.com/InfinitiBit/graphbit/pull/47) ([915eabc](https://github.com/InfinitiBit/graphbit/commit/915eabc0)) on 2025-07-21

### 🐛 Bug Fixes

- Infinite loop issue from character splitter by @tanimahossain-infiniti in [#151](https://github.com/InfinitiBit/graphbit/pull/151) ([936f549](https://github.com/InfinitiBit/graphbit/commit/936f5494)) on 2025-09-05
- Sentence splitter issue by @tanimahossain-infiniti in [#148](https://github.com/InfinitiBit/graphbit/pull/148) ([fca76f9](https://github.com/InfinitiBit/graphbit/commit/fca76f9c)) on 2025-09-05
- Refactored and fixed validation md by @tanima-hossain ([ad38de7](https://github.com/InfinitiBit/graphbit/commit/ad38de7e)) on 2025-08-20
- Anthropic llm config issue fixed by @tanimahossain-infiniti in [#110](https://github.com/InfinitiBit/graphbit/pull/110) ([0d0e0b1](https://github.com/InfinitiBit/graphbit/commit/0d0e0b1e)) on 2025-08-18
- Connection edge cases, unique IDs, and cyclic workflow checks by @jaid-jashim in [#102](https://github.com/InfinitiBit/graphbit/pull/102) ([f9bb183](https://github.com/InfinitiBit/graphbit/commit/f9bb1835)) on 2025-08-15
- Title error fix. by @junaidhossain04 ([71b2935](https://github.com/InfinitiBit/graphbit/commit/71b29359)) on 2025-08-14
- Make GraphBit benchmark CPU affinity logic cross-platform (macOS support) by @jj-devhub in [#51](https://github.com/InfinitiBit/graphbit/pull/51) ([f7d97f7](https://github.com/InfinitiBit/graphbit/commit/f7d97f7e)) on 2025-07-21
- Add macOS fallback for sched_getaffinity by @jj-devhub in [#50](https://github.com/InfinitiBit/graphbit/pull/50) ([11dbbae](https://github.com/InfinitiBit/graphbit/commit/11dbbae2)) on 2025-07-21
- Updated black and dependency check by @yeahia-sarker in [#48](https://github.com/InfinitiBit/graphbit/pull/48) ([7c62b2b](https://github.com/InfinitiBit/graphbit/commit/7c62b2bd)) on 2025-07-21
- **pre-commit**: Resolve hook failures and improve security compliance by @jj-devhub in [#45](https://github.com/InfinitiBit/graphbit/pull/45) ([b8e6294](https://github.com/InfinitiBit/graphbit/commit/b8e6294b)) on 2025-07-17

### ⚡ Performance

- Implement comprehensive Cargo.toml optimization with seamless Maturin integration - Add unified workspace dependency management across all crates - Implement multiple build profiles (dev, release, release-python) with size optimizations - Add cross-platform compatibility with Windows/Unix-specific configurations - Integrate enterprise-grade linting with 103+ quality rules - Optimize Python bindings with Maturin for 25% smaller wheels - Achieve 50% faster builds through workspace dependency caching - Add production-ready Python wheel generation (2.93MB optimized) - Maintain full backward compatibility for all existing Python development workflows by @jj-devhub ([c122ba9](https://github.com/InfinitiBit/graphbit/commit/c122ba9c)) on 2025-09-08
- Performance.md validated by @tanimahossain-infiniti in [#128](https://github.com/InfinitiBit/graphbit/pull/128) ([5734fb9](https://github.com/InfinitiBit/graphbit/commit/5734fb9e)) on 2025-08-25
- AI LLM Multi-Agent Framework Benchmark Comparison Performance Report Summary Across Intel and AMD Virtual Machines by @jaid-jashim in [#65](https://github.com/InfinitiBit/graphbit/pull/65) ([aabaa6a](https://github.com/InfinitiBit/graphbit/commit/aabaa6ae)) on 2025-07-27

### 🗑️ Removed

- Refactor] removed unnecessary packages from pyproject toml by @jaid-jashim in [#116](https://github.com/InfinitiBit/graphbit/pull/116) ([6921160](https://github.com/InfinitiBit/graphbit/commit/69211603)) on 2025-08-19
- Removed Node.condition by @tanima-hossain ([0c0f6f1](https://github.com/InfinitiBit/graphbit/commit/0c0f6f1d)) on 2025-08-18
- Removed doc site and other relevant files and codes by @jaid-jashim in [#101](https://github.com/InfinitiBit/graphbit/pull/101) ([a8c3dc8](https://github.com/InfinitiBit/graphbit/commit/a8c3dc8b)) on 2025-08-14
- Delete Astra DB test integration by @masbul-haider-ovi ([c4acc0f](https://github.com/InfinitiBit/graphbit/commit/c4acc0f7)) on 2025-08-11

### 📚 Documentation

- Root readme architecture diagram by @junaid-hossain in [#140](https://github.com/InfinitiBit/graphbit/pull/140) ([ead004c](https://github.com/InfinitiBit/graphbit/commit/ead004c7)) on 2025-09-03
- About file added by @tanimahossain-infiniti in [#89](https://github.com/InfinitiBit/graphbit/pull/89) ([013cbf4](https://github.com/InfinitiBit/graphbit/commit/013cbf44)) on 2025-09-03
- Framework benchmark report updated. by @junaid-hossain in [#134](https://github.com/InfinitiBit/graphbit/pull/134) ([5dbaa8e](https://github.com/InfinitiBit/graphbit/commit/5dbaa8ee)) on 2025-09-02
- Benchmark readme update. by @junaid-hossain in [#135](https://github.com/InfinitiBit/graphbit/pull/135) ([bb615cf](https://github.com/InfinitiBit/graphbit/commit/bb615cf8)) on 2025-09-02
- Updated root readme by @junaid-hossain in [#132](https://github.com/InfinitiBit/graphbit/pull/132) ([d184769](https://github.com/InfinitiBit/graphbit/commit/d184769b)) on 2025-09-02
- Validated reliability.md file by @tanimahossain-infiniti in [#122](https://github.com/InfinitiBit/graphbit/pull/122) ([5030fe0](https://github.com/InfinitiBit/graphbit/commit/5030fe0d)) on 2025-08-25
- Document loaders documentation. by @junaid-hossain in [#130](https://github.com/InfinitiBit/graphbit/pull/130) ([c96b9d6](https://github.com/InfinitiBit/graphbit/commit/c96b9d6f)) on 2025-08-25
- Agents.md validated by @junaid-hossain in [#111](https://github.com/InfinitiBit/graphbit/pull/111) ([d0770bc](https://github.com/InfinitiBit/graphbit/commit/d0770bc3)) on 2025-08-25
- Monitoring.md by @junaid-hossain in [#124](https://github.com/InfinitiBit/graphbit/pull/124) ([dbd4cdc](https://github.com/InfinitiBit/graphbit/commit/dbd4cdcd)) on 2025-08-22
- Concepts.md, embeddings.md and async-vs-sync.md. by @junaid-hossain in [#114](https://github.com/InfinitiBit/graphbit/pull/114) ([c52873c](https://github.com/InfinitiBit/graphbit/commit/c52873cc)) on 2025-08-22
- GCP integration with Graphbit. by @junaid-hossain in [#70](https://github.com/InfinitiBit/graphbit/pull/70) ([a77ea7f](https://github.com/InfinitiBit/graphbit/commit/a77ea7fb)) on 2025-08-21
- Update CHANGELOG.md by @tanimahossain-infiniti in [#96](https://github.com/InfinitiBit/graphbit/pull/96) ([72b81f7](https://github.com/InfinitiBit/graphbit/commit/72b81f75)) on 2025-08-19
- Updated root readme file. by @junaid-hossain in [#57](https://github.com/InfinitiBit/graphbit/pull/57) ([e67732f](https://github.com/InfinitiBit/graphbit/commit/e67732f6)) on 2025-08-18
- Added a updated version copy of ai framework comparison report by @jaid-jashim in [#72](https://github.com/InfinitiBit/graphbit/pull/72) ([2e2c098](https://github.com/InfinitiBit/graphbit/commit/2e2c0981)) on 2025-08-14
- Updated Documentation of Azure integration with Graphbit. by @junaidhossain04 ([c72660b](https://github.com/InfinitiBit/graphbit/commit/c72660b4)) on 2025-08-14
- Update AstraDB documentation by @masbul-haider-ovi ([4d98dac](https://github.com/InfinitiBit/graphbit/commit/4d98dace)) on 2025-08-14
- Elasticsearch Integration [Connector] by @tanimahossain-infiniti in [#77](https://github.com/InfinitiBit/graphbit/pull/77) ([350cb3b](https://github.com/InfinitiBit/graphbit/commit/350cb3bc)) on 2025-08-08
- IBM Db2 integration with graphbit. by @junaid-hossain in [#76](https://github.com/InfinitiBit/graphbit/pull/76) ([5030633](https://github.com/InfinitiBit/graphbit/commit/50306331)) on 2025-08-08
- Add complete MkDocs documentation site using Material theme by @jaid-jashim in [#79](https://github.com/InfinitiBit/graphbit/pull/79) ([8830fec](https://github.com/InfinitiBit/graphbit/commit/8830fec8)) on 2025-07-31
- Weaviate Integration with Graphbit by @tanimahossain-infiniti in [#75](https://github.com/InfinitiBit/graphbit/pull/75) ([97f67fb](https://github.com/InfinitiBit/graphbit/commit/97f67fbc)) on 2025-07-29
- Milvus Integration with Graphbit by @tanimahossain-infiniti in [#74](https://github.com/InfinitiBit/graphbit/pull/74) ([607c7da](https://github.com/InfinitiBit/graphbit/commit/607c7dae)) on 2025-07-29
- FAISS Integration by @tanimahossain-infiniti in [#60](https://github.com/InfinitiBit/graphbit/pull/60) ([31aa17a](https://github.com/InfinitiBit/graphbit/commit/31aa17a1)) on 2025-07-29
- Documentation: Azure integration with GraphBit. by @junaidhossain04 ([c29fa55](https://github.com/InfinitiBit/graphbit/commit/c29fa55b)) on 2025-07-29
- AWS boto3 integration with Graphbit. by @junaid-hossain in [#63](https://github.com/InfinitiBit/graphbit/pull/63) ([e1dc8bf](https://github.com/InfinitiBit/graphbit/commit/e1dc8bf2)) on 2025-07-27
- Created async-vs-sync.md file. by @junaid-hossain in [#62](https://github.com/InfinitiBit/graphbit/pull/62) ([c8676c0](https://github.com/InfinitiBit/graphbit/commit/c8676c08)) on 2025-07-26
- Chromadb Integration with GraphBit by @tanimahossain-infiniti in [#59](https://github.com/InfinitiBit/graphbit/pull/59) ([8aaf323](https://github.com/InfinitiBit/graphbit/commit/8aaf3231)) on 2025-07-26
- Mariadb basic integration with GraphBit by @tanimahossain-infiniti in [#56](https://github.com/InfinitiBit/graphbit/pull/56) ([1de6cc9](https://github.com/InfinitiBit/graphbit/commit/1de6cc90)) on 2025-07-23
- Updated embeddings documentation file by @junaid-hossain in [#58](https://github.com/InfinitiBit/graphbit/pull/58) ([00920b6](https://github.com/InfinitiBit/graphbit/commit/00920b6c)) on 2025-07-23
- Added pgvector documentation as connector by @junaid-hossain in [#55](https://github.com/InfinitiBit/graphbit/pull/55) ([d45f6e2](https://github.com/InfinitiBit/graphbit/commit/d45f6e2c)) on 2025-07-23
- Added qdrant integration documentation by @tanimahossain-infiniti in [#54](https://github.com/InfinitiBit/graphbit/pull/54) ([0061028](https://github.com/InfinitiBit/graphbit/commit/00610281)) on 2025-07-22
- Added pinecone integration documentation by @tanimahossain-infiniti in [#44](https://github.com/InfinitiBit/graphbit/pull/44) ([3e4a657](https://github.com/InfinitiBit/graphbit/commit/3e4a6575)) on 2025-07-22
- MongoDB integration with graphbit by @junaid-hossain in [#53](https://github.com/InfinitiBit/graphbit/pull/53) ([238cae2](https://github.com/InfinitiBit/graphbit/commit/238cae2a)) on 2025-07-22

### 🧪 Testing

- Fix unit test bugs by @masbulhaider in [#145](https://github.com/InfinitiBit/graphbit/pull/145) ([82a2bbf](https://github.com/InfinitiBit/graphbit/commit/82a2bbfa)) on 2025-09-05
- Integration tests for doc load, workflow and error by @masbulhaider in [#123](https://github.com/InfinitiBit/graphbit/pull/123) ([daf5328](https://github.com/InfinitiBit/graphbit/commit/daf53285)) on 2025-09-03
- Unit test for types, error in rust by @masbulhaider in [#125](https://github.com/InfinitiBit/graphbit/pull/125) ([ed37712](https://github.com/InfinitiBit/graphbit/commit/ed37712b)) on 2025-09-03
- Unit tests for document loader & serialization by @masbulhaider in [#127](https://github.com/InfinitiBit/graphbit/pull/127) ([e32bec0](https://github.com/InfinitiBit/graphbit/commit/e32bec0e)) on 2025-09-03
- Test rust adds extended unit tests for agent, concurrency & graph by @masbulhaider in [#126](https://github.com/InfinitiBit/graphbit/pull/126) ([614bd1f](https://github.com/InfinitiBit/graphbit/commit/614bd1f2)) on 2025-09-03
- Test rust : add document loader, embedding, text splitter unit tests by @masbulhaider in [#106](https://github.com/InfinitiBit/graphbit/pull/106) ([ea87a03](https://github.com/InfinitiBit/graphbit/commit/ea87a030)) on 2025-09-03
- Tests rust: add type, validation & error unit tests by @masbulhaider in [#105](https://github.com/InfinitiBit/graphbit/pull/105) ([f00fdc1](https://github.com/InfinitiBit/graphbit/commit/f00fdc11)) on 2025-09-03
- LLM, agent, workflow tests; refactored validation & workflow integration by @masbulhaider in [#104](https://github.com/InfinitiBit/graphbit/pull/104) ([b3b5a55](https://github.com/InfinitiBit/graphbit/commit/b3b5a55e)) on 2025-09-03
- **py**: Add unit tests for import, init, security & workflow failures by @masbulhaider in [#117](https://github.com/InfinitiBit/graphbit/pull/117) ([c609c43](https://github.com/InfinitiBit/graphbit/commit/c609c431)) on 2025-08-31
- Unit tests for edge cases, client & configuration failure by @masbulhaider in [#113](https://github.com/InfinitiBit/graphbit/pull/113) ([3f97684](https://github.com/InfinitiBit/graphbit/commit/3f976848)) on 2025-08-31
- Added basic unit testing by @masbulhaider in [#95](https://github.com/InfinitiBit/graphbit/pull/95) ([b60003c](https://github.com/InfinitiBit/graphbit/commit/b60003cf)) on 2025-08-31
- Unit tests for doc processing, text splitting, embedding by @masbulhaider in [#94](https://github.com/InfinitiBit/graphbit/pull/94) ([c1fb531](https://github.com/InfinitiBit/graphbit/commit/c1fb5313)) on 2025-08-31
- Unit tests LLM clients coverage; align workflow connect tests by @masbulhaider in [#93](https://github.com/InfinitiBit/graphbit/pull/93) ([32513a2](https://github.com/InfinitiBit/graphbit/commit/32513a24)) on 2025-08-31
- Python tests : add and improve LLM, executor, and document loader integration tests by @masbulhaider in [#90](https://github.com/InfinitiBit/graphbit/pull/90) ([95f6343](https://github.com/InfinitiBit/graphbit/commit/95f6343c)) on 2025-08-14
- Add Astra DB integration test file by @masbul-haider-ovi ([a53bb7e](https://github.com/InfinitiBit/graphbit/commit/a53bb7ec)) on 2025-08-11

### 🔧 Maintenance

- All the benchmark scipts file. by @junaid-hossain in [#133](https://github.com/InfinitiBit/graphbit/pull/133) ([7076cde](https://github.com/InfinitiBit/graphbit/commit/7076cde5)) on 2025-08-30
- Getting-started folder. by @junaid-hossain in [#108](https://github.com/InfinitiBit/graphbit/pull/108) ([f220d27](https://github.com/InfinitiBit/graphbit/commit/f220d274)) on 2025-08-25
- Refactored and validated. by @junaid-hossain in [#121](https://github.com/InfinitiBit/graphbit/pull/121) ([dc44712](https://github.com/InfinitiBit/graphbit/commit/dc44712e)) on 2025-08-22
- Refactored import graphbit in benchmark by @tanimahossain-infiniti in [#100](https://github.com/InfinitiBit/graphbit/pull/100) ([bece4b8](https://github.com/InfinitiBit/graphbit/commit/bece4b82)) on 2025-08-22
- Validated readme and refactored by @tanima-hossain ([207c869](https://github.com/InfinitiBit/graphbit/commit/207c8699)) on 2025-08-19
- Refactored executors examples readme by @tanima-hossain ([355c8b4](https://github.com/InfinitiBit/graphbit/commit/355c8b4e)) on 2025-08-19
- Refactored executors and connectors format by @tanima-hossain ([8e48a38](https://github.com/InfinitiBit/graphbit/commit/8e48a38e)) on 2025-08-19
- Refactored node-types.md file. by @junaidhossain04 ([985bd32](https://github.com/InfinitiBit/graphbit/commit/985bd328)) on 2025-08-15
- Refactored graphbit import by @tanima-hossain ([3f73989](https://github.com/InfinitiBit/graphbit/commit/3f73989d)) on 2025-08-14
- Refactored configuration.md file. by @junaidhossain04 ([beec6f0](https://github.com/InfinitiBit/graphbit/commit/beec6f00)) on 2025-08-14
- Formatted the codebase for clippy compatibility. by @junaid-hossain in [#49](https://github.com/InfinitiBit/graphbit/pull/49) ([4a26a80](https://github.com/InfinitiBit/graphbit/commit/4a26a804)) on 2025-07-21

### 🔄 Others

- Dynamics-graph.md by @junaid-hossain in [#119](https://github.com/InfinitiBit/graphbit/pull/119) ([6a00ec3](https://github.com/InfinitiBit/graphbit/commit/6a00ec31)) on 2025-08-22
- Workflow builder readme validated by @tanimahossain-infiniti in [#112](https://github.com/InfinitiBit/graphbit/pull/112) ([d721034](https://github.com/InfinitiBit/graphbit/commit/d721034b)) on 2025-08-22
- Slight changes made. by @junaidhossain04 ([a420e33](https://github.com/InfinitiBit/graphbit/commit/a420e33d)) on 2025-08-19
- Graphbit init call auto by default by @jaid-jashim in [#115](https://github.com/InfinitiBit/graphbit/pull/115) ([62318f4](https://github.com/InfinitiBit/graphbit/commit/62318f42)) on 2025-08-19
- Docs examples folder read me checked by @tanima-hossain ([ca6377c](https://github.com/InfinitiBit/graphbit/commit/ca6377c7)) on 2025-08-18
- Removal of the script. by @junaidhossain04 ([fd0f905](https://github.com/InfinitiBit/graphbit/commit/fd0f9058)) on 2025-08-14
- Typo in codes by @tanima-hossain ([e47662f](https://github.com/InfinitiBit/graphbit/commit/e47662fc)) on 2025-08-13
- Changed title by @tanima-hossain ([ae9fa0d](https://github.com/InfinitiBit/graphbit/commit/ae9fa0d3)) on 2025-08-13
- Tutorial: AstraDB integration with graphbit. by @masbul-haider-ovi ([5766d32](https://github.com/InfinitiBit/graphbit/commit/5766d325)) on 2025-08-11
- Feature/docs site by @jaid-jashim in [#83](https://github.com/InfinitiBit/graphbit/pull/83) ([a5074b9](https://github.com/InfinitiBit/graphbit/commit/a5074b96)) on 2025-08-09

---
**Total Changes**: 105
**Changes by Category**: ✨ New Features: 20 | 🐛 Bug Fixes: 10 | ⚡ Performance: 3 | 🗑️ Removed: 4 | 📚 Documentation: 32 | 🧪 Testing: 15 | 🔧 Maintenance: 11 | 🔄 Others: 10

## [0.4.0] - 2025-09-08

### ✨ New Features

- Test rust : add python binding tests, helper functions ([#107](https://github.com/InfinitiBit/graphbit/pull/107))
- Added tool calling support ([#131](https://github.com/InfinitiBit/graphbit/pull/131))
- Pr review addressed
- Review addressed
- Updated python-api.md
- Updated configuration.md
- Refactored and updated python-api.md file.
- Update astradb_integration.md
- Added nodejs binding ([#97](https://github.com/InfinitiBit/graphbit/pull/97))
- : Rust Core, Rust Python Binding: Agentic Workflow with Dep-Batching and Parent Preamble #87 ([#87](https://github.com/InfinitiBit/graphbit/pull/87))
- Updated the readme file in python folder.
- Updated readme
- Added document loader support ([#84](https://github.com/InfinitiBit/graphbit/pull/84))
- Added SECURITY.md ([#68](https://github.com/InfinitiBit/graphbit/pull/68))
- Added perplexity support ([#67](https://github.com/InfinitiBit/graphbit/pull/67))
- Added deepseek support ([#66](https://github.com/InfinitiBit/graphbit/pull/66))
- Implement LLM-Graphbit-Playwright Browser Automation Agent ([#52](https://github.com/InfinitiBit/graphbit/pull/52))
- Added textsplitter ([#64](https://github.com/InfinitiBit/graphbit/pull/64))
- Added chatbot development example ([#38](https://github.com/InfinitiBit/graphbit/pull/38))
- Integrated google search api with graphbit ([#47](https://github.com/InfinitiBit/graphbit/pull/47))

### 🐛 Bug Fixes

- Infinite loop issue from character splitter ([#151](https://github.com/InfinitiBit/graphbit/pull/151))
- Sentence splitter issue ([#148](https://github.com/InfinitiBit/graphbit/pull/148))
- Refactored and fixed validation md
- Anthropic llm config issue fixed ([#110](https://github.com/InfinitiBit/graphbit/pull/110))
- Connection edge cases, unique IDs, and cyclic workflow checks ([#102](https://github.com/InfinitiBit/graphbit/pull/102))
- Title error fix.
- Make GraphBit benchmark CPU affinity logic cross-platform (macOS support) ([#51](https://github.com/InfinitiBit/graphbit/pull/51))
- Add macOS fallback for sched_getaffinity ([#50](https://github.com/InfinitiBit/graphbit/pull/50))
- Updated black and dependency check ([#48](https://github.com/InfinitiBit/graphbit/pull/48))
- **pre-commit**: Resolve hook failures and improve security compliance ([#45](https://github.com/InfinitiBit/graphbit/pull/45))

### ⚡ Performance

- Implement comprehensive Cargo.toml optimization with seamless Maturin integration - Add unified workspace dependency management across all crates - Implement multiple build profiles (dev, release, release-python) with size optimizations - Add cross-platform compatibility with Windows/Unix-specific configurations - Integrate enterprise-grade linting with 103+ quality rules - Optimize Python bindings with Maturin for 25% smaller wheels - Achieve 50% faster builds through workspace dependency caching - Add production-ready Python wheel generation (2.93MB optimized) - Maintain full backward compatibility for all existing Python development workflows
- Performance.md validated ([#128](https://github.com/InfinitiBit/graphbit/pull/128))
- AI LLM Multi-Agent Framework Benchmark Comparison Performance Report Summary Across Intel and AMD Virtual Machines ([#65](https://github.com/InfinitiBit/graphbit/pull/65))

### 🗑️ Removed

- Refactor] removed unnecessary packages from pyproject toml ([#116](https://github.com/InfinitiBit/graphbit/pull/116))
- Removed Node.condition
- Removed doc site and other relevant files and codes ([#101](https://github.com/InfinitiBit/graphbit/pull/101))
- Delete Astra DB test integration

### 📚 Documentation

- Root readme architecture diagram ([#140](https://github.com/InfinitiBit/graphbit/pull/140))
- About file added ([#89](https://github.com/InfinitiBit/graphbit/pull/89))
- Framework benchmark report updated. ([#134](https://github.com/InfinitiBit/graphbit/pull/134))
- Benchmark readme update. ([#135](https://github.com/InfinitiBit/graphbit/pull/135))
- Updated root readme ([#132](https://github.com/InfinitiBit/graphbit/pull/132))
- Validated reliability.md file ([#122](https://github.com/InfinitiBit/graphbit/pull/122))
- Document loaders documentation. ([#130](https://github.com/InfinitiBit/graphbit/pull/130))
- Agents.md validated ([#111](https://github.com/InfinitiBit/graphbit/pull/111))
- Monitoring.md ([#124](https://github.com/InfinitiBit/graphbit/pull/124))
- Concepts.md, embeddings.md and async-vs-sync.md. ([#114](https://github.com/InfinitiBit/graphbit/pull/114))
- GCP integration with Graphbit. ([#70](https://github.com/InfinitiBit/graphbit/pull/70))
- Update CHANGELOG.md ([#96](https://github.com/InfinitiBit/graphbit/pull/96))
- Updated root readme file. ([#57](https://github.com/InfinitiBit/graphbit/pull/57))
- Added a updated version copy of ai framework comparison report ([#72](https://github.com/InfinitiBit/graphbit/pull/72))
- Updated Documentation of Azure integration with Graphbit.
- Update AstraDB documentation
- Elasticsearch Integration [Connector] ([#77](https://github.com/InfinitiBit/graphbit/pull/77))
- IBM Db2 integration with graphbit. ([#76](https://github.com/InfinitiBit/graphbit/pull/76))
- Add complete MkDocs documentation site using Material theme ([#79](https://github.com/InfinitiBit/graphbit/pull/79))
- Weaviate Integration with Graphbit ([#75](https://github.com/InfinitiBit/graphbit/pull/75))
- Milvus Integration with Graphbit ([#74](https://github.com/InfinitiBit/graphbit/pull/74))
- FAISS Integration ([#60](https://github.com/InfinitiBit/graphbit/pull/60))
- Documentation: Azure integration with GraphBit.
- AWS boto3 integration with Graphbit. ([#63](https://github.com/InfinitiBit/graphbit/pull/63))
- Created async-vs-sync.md file. ([#62](https://github.com/InfinitiBit/graphbit/pull/62))
- Chromadb Integration with GraphBit ([#59](https://github.com/InfinitiBit/graphbit/pull/59))
- Mariadb basic integration with GraphBit ([#56](https://github.com/InfinitiBit/graphbit/pull/56))
- Updated embeddings documentation file ([#58](https://github.com/InfinitiBit/graphbit/pull/58))
- Added pgvector documentation as connector ([#55](https://github.com/InfinitiBit/graphbit/pull/55))
- Added qdrant integration documentation ([#54](https://github.com/InfinitiBit/graphbit/pull/54))
- Added pinecone integration documentation ([#44](https://github.com/InfinitiBit/graphbit/pull/44))
- MongoDB integration with graphbit ([#53](https://github.com/InfinitiBit/graphbit/pull/53))

### 🧪 Testing

- Fix unit test bugs ([#145](https://github.com/InfinitiBit/graphbit/pull/145))
- Integration tests for doc load, workflow and error ([#123](https://github.com/InfinitiBit/graphbit/pull/123))
- Unit test for types, error in rust ([#125](https://github.com/InfinitiBit/graphbit/pull/125))
- Unit tests for document loader & serialization ([#127](https://github.com/InfinitiBit/graphbit/pull/127))
- Test rust adds extended unit tests for agent, concurrency & graph ([#126](https://github.com/InfinitiBit/graphbit/pull/126))
- Test rust : add document loader, embedding, text splitter unit tests ([#106](https://github.com/InfinitiBit/graphbit/pull/106))
- Tests rust: add type, validation & error unit tests ([#105](https://github.com/InfinitiBit/graphbit/pull/105))
- LLM, agent, workflow tests; refactored validation & workflow integration ([#104](https://github.com/InfinitiBit/graphbit/pull/104))
- **py**: Add unit tests for import, init, security & workflow failures ([#117](https://github.com/InfinitiBit/graphbit/pull/117))
- Unit tests for edge cases, client & configuration failure ([#113](https://github.com/InfinitiBit/graphbit/pull/113))
- Added basic unit testing ([#95](https://github.com/InfinitiBit/graphbit/pull/95))
- Unit tests for doc processing, text splitting, embedding ([#94](https://github.com/InfinitiBit/graphbit/pull/94))
- Unit tests LLM clients coverage; align workflow connect tests ([#93](https://github.com/InfinitiBit/graphbit/pull/93))
- Python tests : add and improve LLM, executor, and document loader integration tests ([#90](https://github.com/InfinitiBit/graphbit/pull/90))
- Add Astra DB integration test file

### 🔧 Maintenance

- All the benchmark scipts file. ([#133](https://github.com/InfinitiBit/graphbit/pull/133))
- Getting-started folder. ([#108](https://github.com/InfinitiBit/graphbit/pull/108))
- Dynamics-graph.md ([#119](https://github.com/InfinitiBit/graphbit/pull/119))
- Refactored and validated. ([#121](https://github.com/InfinitiBit/graphbit/pull/121))
- Refactored import graphbit in benchmark ([#100](https://github.com/InfinitiBit/graphbit/pull/100))
- Workflow builder readme validated ([#112](https://github.com/InfinitiBit/graphbit/pull/112))
- Validated readme and refactored
- Slight changes made.
- Graphbit init call auto by default ([#115](https://github.com/InfinitiBit/graphbit/pull/115))
- Refactored executors examples readme
- Refactored executors and connectors format
- Docs examples folder read me checked
- Refactored node-types.md file.
- Refactored graphbit import
- Refactored configuration.md file.
- Removal of the script.
- Typo in codes
- Changed title
- Tutorial: AstraDB integration with graphbit.
- Feature/docs site ([#83](https://github.com/InfinitiBit/graphbit/pull/83))
- Formatted the codebase for clippy compatibility. ([#49](https://github.com/InfinitiBit/graphbit/pull/49))

---
**Total Changes**: 105
- Complete MkDocs site (Material theme)
- AI LLM multi-agent benchmark report summary
- Cross-platform CPU affinity logic with macOS fallback
- Resolved pre-commit hook failures; improved security compliance
- Updated black and dependency checks; codebase reformatted


## [0.3.0-alpha] – 2025-07-16

- Comprehensive Python tests added
- Rust integration test coverage improved
- Improved benchmark runner
- Benchmark run consistency improved
- Centralized control of benchmark run counts
- Centralized run configuration committed
- Explicit flags/config for run counts
- Explicit run control documented
- Dockerization support for benchmark
- Production volume mount paths refined
- Tarpaulin coverage added for Rust
- Tarpaulin configuration integrated
- Benchmark documentation updated
- Root README updated
- Contributing guidelines updated


## [0.2.0-alpha] – 2025-06-28

- Ollama Python support added
- LangGraph integrated into benchmark framework
- CrewAI benchmark scenarios optimized for performance and reliability
- Performance optimizations
- Benchmark and Python binding refactors
- New integration tests added
- Python integration tests expanded
- Fixed Python integration tests for GitHub Actions
- Python examples for GraphBit added
- Benchmark evaluation updated
- Example code updated
- Python documentation updated
- Pre-commit issues resolved
- Pre-test commit fixed for all files
- Makefile fixes
- Root README updated
- GitHub Actions workflow removed


## [0.1.0] - 2025-06-11

- Initial GraphBit release: declarative agentic workflow framework
- Modular architecture with core modules (agents, llm, graph, validation, workflow, types, errors)
- Multi-LLM support: OpenAI GPT, Anthropic Claude, Ollama, extensible providers
- Graph-based workflows (DAG), dependency management, topological execution
- Node types: agent, transform, conditional
- Parallel execution with configurable concurrency
- Async/await support throughout
- Full Python API via PyO3 with async support
- Strong typing and validation
- JSON schema validation for LLM outputs
- UUID identifiers for all components
- Intelligent dependency resolution
- Error handling: retries with backoff, comprehensive errors, failure recovery
- Usage tracking: tokens, cost estimation, performance metrics
- CLI (graphbit): init, validate, run, config, debug/verbose
- JSON workflow configs with env var support and custom files
- Integration examples: FastAPI, Django, Jupyter
- Documentation and examples: README, Ollama guide, testing/benchmarking, Python and Rust API docs
- Testing: unit/integration, benchmarking suite, mock LLM providers, CI/CD
- Dependencies and MSRV: tokio, serde, anyhow/thiserror, uuid, petgraph, reqwest, clap, pyo3, chrono; Rust 1.70+
