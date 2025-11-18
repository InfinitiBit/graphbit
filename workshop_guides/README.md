# Non-Ollama GraphBit ParallelRAG Workshop Documentation

**Purpose**: Comprehensive workshop materials for live technical demonstrations  
**Last Updated**: November 17, 2025  
**Target Audience**: Developers, Architects, Data Scientists  
**Scope**: Cloud-based LLM providers (OpenAI, Anthropic) - **EXCLUDES** Ollama integration

---

## 📚 Documentation Structure

This directory contains complete workshop documentation for demonstrating GraphBit's ParallelRAG capabilities with cloud-based LLM providers.

### Main Documentation Files

1. **[NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md](NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md)** (729 lines)
   - Complete workshop guide (45-60 minutes)
   - 4 demonstration scenarios
   - Prerequisites and setup instructions
   - Code walkthroughs and talking points
   - Troubleshooting guide

2. **[file_inventory.md](file_inventory.md)** (120 lines)
   - Complete inventory of all non-Ollama ParallelRAG files
   - 21 files categorized by type and purpose
   - File metadata (lines of code, LLM providers, etc.)
   - Quick reference table

3. **[command_reference.md](command_reference.md)** (150 lines)
   - Quick reference for all executable commands
   - Organized by use case (Quick Demo, Comparison, Stress Test, Visualization)
   - Copy-paste ready examples
   - Runtime estimates and API cost information

4. **[code_architecture_map.md](code_architecture_map.md)** (657 lines)
   - Detailed code architecture documentation
   - File-by-file breakdown with line numbers
   - Key classes, functions, and code sections
   - Entry points and configuration details

### Demo Scripts

Executable shell scripts for each demonstration:

1. **[demo_scripts/quick_demo.sh](demo_scripts/quick_demo.sh)**
   - 5-minute quick demo
   - Runs `parallelrag_core/examples/parallel_rag_optimized.py`
   - Shows GIL-releasing architecture

2. **[demo_scripts/comparison_demo.sh](demo_scripts/comparison_demo.sh)**
   - 10-minute framework comparison
   - GraphBit vs LangChain
   - Performance metrics and speedup calculations

3. **[demo_scripts/scalability_demo.sh](demo_scripts/scalability_demo.sh)**
   - 15-minute scalability demo
   - Progressive load testing (100-1000 docs)
   - Worker scaling analysis

4. **[demo_scripts/visualization_demo.sh](demo_scripts/visualization_demo.sh)**
   - 5-minute visualization demo
   - Generates 12 performance charts
   - Publication-ready PNG outputs

### Sample Outputs

Pre-generated results for backup during live workshops:

- **[sample_outputs/README.md](sample_outputs/README.md)** - Usage guide for sample outputs
- **JSON Results**: Framework comparison and stress test results
- **Charts**: All 12 performance charts (PNG format)
- **Console Outputs**: Sample console outputs for each demo

---

## 🚀 Quick Start

### For Workshop Presenters

1. **Review the main guide**:
   ```bash
   cat workshop_guides/NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md
   ```

2. **Test all demos**:
   ```bash
   # Set API key
   export OPENAI_API_KEY="sk-your-api-key-here"
   
   # Run each demo script
   bash workshop_guides/demo_scripts/quick_demo.sh
   bash workshop_guides/demo_scripts/comparison_demo.sh
   bash workshop_guides/demo_scripts/scalability_demo.sh
   bash workshop_guides/demo_scripts/visualization_demo.sh
   ```

3. **Pre-generate sample outputs** (backup plan):
   ```bash
   # Run benchmarks to generate JSON results
   python parallelrag_core/tests/benchmarks/benchmark_framework_comparison.py --framework both --max-docs 100
   
   # Generate charts
   python parallelrag_core/visualizations/create_visualizations.py
   python create_resource_charts.py
   python create_additional_visualizations.py
   
   # Copy to sample_outputs directory
   cp *.json workshop_guides/sample_outputs/
   cp chart_*.png workshop_guides/sample_outputs/
   ```

### For Workshop Attendees

1. **Read the workshop guide**:
   - Start with `NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md`
   - Review prerequisites and setup instructions
   - Follow along with demonstrations

2. **Run demos yourself**:
   - Use the demo scripts in `demo_scripts/`
   - Refer to `command_reference.md` for all commands
   - Check `code_architecture_map.md` for code details

3. **Explore sample outputs**:
   - View pre-generated results in `sample_outputs/`
   - Compare with your own results
   - Use as reference for expected outputs

---

## 📊 Workshop Overview

### Duration
- **Full Workshop**: 45-60 minutes
- **Individual Demos**: 5-15 minutes each

### Demonstrations

| Demo | Duration | API Cost | Prerequisites |
|------|----------|----------|---------------|
| **Demo 1: Quick ParallelRAG** | 5 min | ~$0.01-0.02 | OpenAI API key |
| **Demo 2: Framework Comparison** | 10 min | ~$0.05-0.10 | OpenAI API key |
| **Demo 3: Scalability** | 15 min | NONE (mocked) | None |
| **Demo 4: Visualization** | 5 min | NONE | JSON results |

### Key Takeaways

1. ✅ GraphBit releases the GIL for true parallelism
2. ✅ 10-100x speedup over sequential processing
3. ✅ 1.2-2x faster than LangChain
4. ✅ Production-ready with error handling and monitoring
5. ✅ Works with OpenAI, Anthropic, and other cloud providers

---

## 📝 File Summary

### Documentation Files (4 files, ~1,656 lines)
- Main workshop guide: 729 lines
- File inventory: 120 lines
- Command reference: 150 lines
- Code architecture map: 657 lines

### Demo Scripts (4 files, ~200 lines)
- Quick demo: ~50 lines
- Comparison demo: ~60 lines
- Scalability demo: ~45 lines
- Visualization demo: ~75 lines

### Sample Outputs
- JSON results: 2+ files
- Charts: 12 PNG files
- Console outputs: 4 text files

**Total**: 8+ documentation files, 4 demo scripts, 18+ sample outputs

---

## 🎯 Usage Scenarios

### Scenario 1: Live Technical Workshop
- Use main workshop guide as presentation outline
- Run demo scripts live with API keys
- Show code architecture during walkthroughs
- Use sample outputs as backup if API fails

### Scenario 2: Self-Paced Learning
- Read workshop guide at your own pace
- Run demos yourself with your API keys
- Explore code architecture map for details
- Compare your results with sample outputs

### Scenario 3: Team Training
- Share workshop guide with team
- Run demos together in team meeting
- Discuss code architecture and design decisions
- Use as reference for implementing ParallelRAG

### Scenario 4: Sales/Marketing Demo
- Use quick demo (5 minutes) for high-level overview
- Show visualization charts for visual impact
- Reference performance metrics from sample outputs
- Keep it simple, focus on benefits

---

## 🔗 Related Documentation

### In This Repository
- **Main README**: `../README.md`
- **Ollama Integration**: `../ollama_integration/README.md` (separate guide)
- **Performance Whitepaper**: `../GRAPHBIT_PERFORMANCE_WHITEPAPER.md`
- **Framework Comparison**: `../FRAMEWORK_COMPARISON.md`

### External Resources
- **GraphBit GitHub**: https://github.com/graphbit/graphbit
- **GraphBit Documentation**: https://docs.graphbit.ai
- **OpenAI API Docs**: https://platform.openai.com/docs
- **Anthropic API Docs**: https://docs.anthropic.com

---

## 📧 Support

For questions or feedback about workshop materials:
- **Email**: support@graphbit.ai
- **GitHub Issues**: https://github.com/graphbit/graphbit/issues
- **Discord**: [Join our community]

---

**Last Updated**: November 17, 2025  
**Version**: 1.0  
**Status**: ✅ Complete and Ready for Use


