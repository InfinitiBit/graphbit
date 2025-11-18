#!/bin/bash
# Visualization Demo (5 minutes)
# Generates performance charts from benchmark results

echo "================================================================================"
echo "Demo 4: Visualization Demo"
echo "================================================================================"
echo ""
echo "This demo generates:"
echo "  - 5 main performance charts (create_visualizations.py)"
echo "  - 3 resource utilization charts (create_resource_charts.py)"
echo "  - 4 additional analysis charts (create_additional_visualizations.py)"
echo ""
echo "Runtime: ~1-2 minutes"
echo "API Cost: NONE (uses existing JSON results)"
echo ""
echo "================================================================================"
echo ""

# Check if result files exist
if [ ! -f "graphbit_stress_50k.json" ] || [ ! -f "langchain_stress_50k.json" ]; then
    echo "⚠️  Warning: Required JSON result files not found"
    echo ""
    echo "This demo requires benchmark results from previous runs:"
    echo "  - graphbit_stress_50k.json"
    echo "  - langchain_stress_50k.json"
    echo ""
    echo "Please run the framework comparison demo first:"
    echo "  ./workshop_guides/demo_scripts/comparison_demo.sh"
    echo ""
    echo "Or use sample outputs from workshop_guides/sample_outputs/"
    echo ""
    exit 1
fi

echo "✅ Found required JSON result files"
echo ""

# Generate main performance charts
echo "Step 1: Generating main performance charts..."
echo "Running: python create_visualizations.py"
echo ""
python create_visualizations.py

echo ""
echo "Step 2: Generating resource utilization charts..."
echo "Running: python create_resource_charts.py"
echo ""
python create_resource_charts.py

echo ""
echo "Step 3: Generating additional analysis charts..."
echo "Running: python create_additional_visualizations.py"
echo ""
python create_additional_visualizations.py

echo ""
echo "================================================================================"
echo "Demo 4 Complete!"
echo "================================================================================"
echo ""
echo "Generated charts:"
echo "  Main Performance Charts:"
echo "    - chart_total_time.png"
echo "    - chart_throughput.png"
echo "    - chart_speedup.png"
echo "    - chart_component_breakdown.png"
echo "    - chart_extended_capacity.png"
echo ""
echo "  Resource Utilization Charts:"
echo "    - chart_memory_usage.png"
echo "    - chart_cpu_utilization.png"
echo "    - chart_resource_efficiency.png"
echo ""
echo "  Additional Analysis Charts:"
echo "    - chart_worker_optimization.png"
echo "    - chart_document_size_impact.png"
echo "    - chart_cost_comparison.png"
echo "    - chart_scaling_efficiency.png"
echo ""
echo "Key Takeaways:"
echo "  ✓ GraphBit shows consistent speedup across all scales"
echo "  ✓ Memory usage is efficient and predictable"
echo "  ✓ CPU utilization is high (good parallelism)"
echo "  ✓ Cost savings: 30-50% vs LangChain"
echo ""

