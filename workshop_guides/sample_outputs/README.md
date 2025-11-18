# Sample Outputs for Workshop Demonstrations

**Purpose**: Pre-generated results for workshop demonstrations when live API calls might fail

**Last Updated**: November 17, 2025

---

## 📋 Contents

This directory contains sample outputs for all workshop demonstrations:

### 1. Quick Demo Outputs
- `quick_demo_console_output.txt` - Console output from `parallel_rag_optimized.py`
- Expected runtime: ~2-3 minutes
- Expected API cost: ~$0.01-0.02

### 2. Framework Comparison Outputs
- `framework_comparison_results.json` - JSON results from GraphBit vs LangChain comparison
- `framework_comparison_console_output.txt` - Console output
- Expected runtime: ~5-10 minutes
- Expected API cost: ~$0.05-0.10

### 3. Scalability Demo Outputs
- `stress_test_results.json` - JSON results from progressive load testing
- `stress_test_console_output.txt` - Console output
- Expected runtime: ~10-15 minutes
- Expected API cost: NONE (mocked calls)

### 4. Visualization Demo Outputs
- `chart_total_time.png` - Total Time vs Document Count
- `chart_throughput.png` - Throughput vs Document Count
- `chart_speedup.png` - GraphBit Speedup vs LangChain
- `chart_component_breakdown.png` - Component Time Breakdown
- `chart_extended_capacity.png` - Extended Capacity Results
- `chart_memory_usage.png` - Memory Usage Across Document Scales
- `chart_cpu_utilization.png` - CPU Utilization Patterns
- `chart_resource_efficiency.png` - Throughput per GB Memory
- `chart_worker_optimization.png` - Worker Count Optimization
- `chart_document_size_impact.png` - Document Size Impact
- `chart_cost_comparison.png` - Cost Comparison GraphBit vs LangChain
- `chart_scaling_efficiency.png` - Scaling Efficiency (100-500K docs)

---

## 🎯 Usage

### During Live Workshops

If live API calls fail or are too slow, use these sample outputs:

```bash
# Copy sample outputs to main directory
cp workshop_guides/sample_outputs/*.json .
cp workshop_guides/sample_outputs/*.png .

# Show sample console output
cat workshop_guides/sample_outputs/quick_demo_console_output.txt

# Show sample JSON results
cat workshop_guides/sample_outputs/framework_comparison_results.json | jq
```

### For Offline Demonstrations

All sample outputs can be used for offline demonstrations without API keys:

```bash
# Generate charts from sample JSON
python parallelrag_core/visualizations/create_visualizations.py  # Uses sample JSON files

# Show sample results
cat workshop_guides/sample_outputs/framework_comparison_console_output.txt
```

---

## 📝 Notes

- **Sample outputs are real**: All outputs are from actual benchmark runs
- **Results may vary**: Your results may differ based on hardware, network, and API latency
- **API costs**: Sample outputs help avoid API costs during practice runs
- **Backup plan**: Always have sample outputs ready for live workshops

---

## 🔗 Related Files

- **Main Workshop Guide**: `workshop_guides/NON_OLLAMA_PARALLELRAG_WORKSHOP_GUIDE.md`
- **Demo Scripts**: `workshop_guides/demo_scripts/`
- **Command Reference**: `workshop_guides/command_reference.md`


