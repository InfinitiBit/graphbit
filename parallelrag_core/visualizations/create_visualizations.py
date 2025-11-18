#!/usr/bin/env python3
"""
Create performance visualization charts from JSON test results.
"""

import json
import matplotlib.pyplot as plt
import matplotlib
matplotlib.use('Agg')  # Use non-interactive backend
import numpy as np
from pathlib import Path
from typing import Dict, List, Any

# Set style
plt.style.use('seaborn-v0_8-darkgrid')

def load_json_results(filepath: str) -> Dict[str, Any]:
    """Load JSON results from file."""
    with open(filepath, 'r') as f:
        return json.load(f)

def extract_test_data(results: List[Dict[str, Any]], framework: str) -> tuple:
    """Extract test data from results for a specific framework."""
    doc_counts = []
    total_times = []
    load_times = []
    chunk_times = []
    throughputs = []
    chunks_created = []

    for test in results:
        if test.get(framework):
            data = test[framework]
            doc_counts.append(data['num_documents'])
            total_times.append(data['total_time'])
            load_times.append(data['load_time'])
            chunk_times.append(data['chunk_time'])
            throughputs.append(data['throughput_docs_per_sec'])
            chunks_created.append(data['chunks_created'])

    return doc_counts, total_times, load_times, chunk_times, throughputs, chunks_created

def create_chart1_total_time(graphbit_data, langchain_data, output_file='chart_total_time.png'):
    """Chart 1: Total Time vs Document Count."""
    gb_docs, gb_times, _, _, _, _ = graphbit_data
    lc_docs, lc_times, _, _, _, _ = langchain_data
    
    plt.figure(figsize=(12, 7))
    plt.plot(gb_docs, gb_times, 'o-', linewidth=2, markersize=8, label='GraphBit', color='#2E86AB')
    plt.plot(lc_docs, lc_times, 's-', linewidth=2, markersize=8, label='LangChain', color='#A23B72')
    
    plt.xlabel('Document Count', fontsize=12, fontweight='bold')
    plt.ylabel('Total Time (seconds)', fontsize=12, fontweight='bold')
    plt.title('Total Processing Time vs Document Count', fontsize=14, fontweight='bold')
    plt.legend(fontsize=11)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart2_throughput(graphbit_data, langchain_data, output_file='chart_throughput.png'):
    """Chart 2: Throughput vs Document Count."""
    gb_docs, _, _, _, gb_throughput, _ = graphbit_data
    lc_docs, _, _, _, lc_throughput, _ = langchain_data
    
    plt.figure(figsize=(12, 7))
    plt.plot(gb_docs, gb_throughput, 'o-', linewidth=2, markersize=8, label='GraphBit', color='#2E86AB')
    plt.plot(lc_docs, lc_throughput, 's-', linewidth=2, markersize=8, label='LangChain', color='#A23B72')
    
    plt.xlabel('Document Count', fontsize=12, fontweight='bold')
    plt.ylabel('Throughput (docs/sec)', fontsize=12, fontweight='bold')
    plt.title('Processing Throughput vs Document Count', fontsize=14, fontweight='bold')
    plt.legend(fontsize=11)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart3_speedup(graphbit_data, langchain_data, output_file='chart_speedup.png'):
    """Chart 3: Speedup vs Document Count."""
    gb_docs, gb_times, _, _, _, _ = graphbit_data
    lc_docs, lc_times, _, _, _, _ = langchain_data
    
    # Calculate speedup (LangChain time / GraphBit time)
    speedups = [lc_t / gb_t for gb_t, lc_t in zip(gb_times, lc_times)]
    
    plt.figure(figsize=(12, 7))
    bars = plt.bar(range(len(gb_docs)), speedups, color='#06A77D', edgecolor='black', linewidth=1.5)
    
    # Add value labels on bars
    for i, (bar, speedup) in enumerate(zip(bars, speedups)):
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2., height,
                f'{speedup:.1f}x',
                ha='center', va='bottom', fontsize=10, fontweight='bold')
    
    plt.xlabel('Document Count', fontsize=12, fontweight='bold')
    plt.ylabel('Speedup (LangChain time / GraphBit time)', fontsize=12, fontweight='bold')
    plt.title('GraphBit Speedup vs LangChain', fontsize=14, fontweight='bold')
    plt.xticks(range(len(gb_docs)), [f'{d:,}' for d in gb_docs], rotation=45, ha='right')
    plt.axhline(y=1, color='red', linestyle='--', linewidth=2, label='No speedup (1x)')
    plt.legend(fontsize=11)
    plt.grid(True, alpha=0.3, axis='y')
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart4_component_breakdown(graphbit_data, langchain_data, output_file='chart_component_breakdown.png'):
    """Chart 4: Component Breakdown (stacked bar chart)."""
    gb_docs, _, gb_load, gb_chunk, _, _ = graphbit_data
    lc_docs, _, lc_load, lc_chunk, _, _ = langchain_data
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7))
    
    # GraphBit breakdown
    x = range(len(gb_docs))
    ax1.bar(x, gb_load, label='Load Time', color='#2E86AB')
    ax1.bar(x, gb_chunk, bottom=gb_load, label='Chunk Time', color='#06A77D')
    ax1.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Time (seconds)', fontsize=12, fontweight='bold')
    ax1.set_title('GraphBit - Component Breakdown', fontsize=14, fontweight='bold')
    ax1.set_xticks(x)
    ax1.set_xticklabels([f'{d:,}' for d in gb_docs], rotation=45, ha='right')
    ax1.legend(fontsize=11)
    ax1.grid(True, alpha=0.3, axis='y')
    
    # LangChain breakdown
    ax2.bar(x, lc_load, label='Load Time', color='#A23B72')
    ax2.bar(x, lc_chunk, bottom=lc_load, label='Chunk Time', color='#F18F01')
    ax2.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax2.set_ylabel('Time (seconds)', fontsize=12, fontweight='bold')
    ax2.set_title('LangChain - Component Breakdown', fontsize=14, fontweight='bold')
    ax2.set_xticks(x)
    ax2.set_xticklabels([f'{d:,}' for d in lc_docs], rotation=45, ha='right')
    ax2.legend(fontsize=11)
    ax2.grid(True, alpha=0.3, axis='y')
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart5_extended_capacity(output_file='chart_extended_capacity.png'):
    """Chart 5: Extended Capacity Testing (100K, 250K, 500K)."""
    # Load extended capacity results
    try:
        results_100k = load_json_results('graphbit_max_capacity_100k.json')
        results_250k = load_json_results('graphbit_max_capacity_250k.json')
        results_500k = load_json_results('graphbit_max_capacity_500k.json')
    except FileNotFoundError:
        print("⚠️  Extended capacity JSON files not found, skipping chart 5")
        return

    # Extract data for 100K, 250K, 500K tests
    doc_counts = [100000, 250000, 500000]
    total_times = []
    throughputs = []

    for results, target_count in zip([results_100k, results_250k, results_500k], doc_counts):
        for test in results:
            if test.get('graphbit') and test['graphbit']['num_documents'] == target_count:
                total_times.append(test['graphbit']['total_time'])
                throughputs.append(test['graphbit']['throughput_docs_per_sec'])
                break
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7))
    
    # Total time
    ax1.bar(range(len(doc_counts)), total_times, color='#2E86AB', edgecolor='black', linewidth=1.5)
    ax1.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Total Time (seconds)', fontsize=12, fontweight='bold')
    ax1.set_title('GraphBit Extended Capacity - Total Time', fontsize=14, fontweight='bold')
    ax1.set_xticks(range(len(doc_counts)))
    ax1.set_xticklabels([f'{d:,}' for d in doc_counts])
    ax1.grid(True, alpha=0.3, axis='y')
    
    # Add value labels
    for i, (time, throughput) in enumerate(zip(total_times, throughputs)):
        ax1.text(i, time, f'{time:.1f}s', ha='center', va='bottom', fontsize=10, fontweight='bold')
    
    # Throughput
    ax2.bar(range(len(doc_counts)), throughputs, color='#06A77D', edgecolor='black', linewidth=1.5)
    ax2.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax2.set_ylabel('Throughput (docs/sec)', fontsize=12, fontweight='bold')
    ax2.set_title('GraphBit Extended Capacity - Throughput', fontsize=14, fontweight='bold')
    ax2.set_xticks(range(len(doc_counts)))
    ax2.set_xticklabels([f'{d:,}' for d in doc_counts])
    ax2.grid(True, alpha=0.3, axis='y')
    
    # Add value labels
    for i, throughput in enumerate(throughputs):
        ax2.text(i, throughput, f'{throughput:.1f}', ha='center', va='bottom', fontsize=10, fontweight='bold')
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def main():
    """Main function to create all visualizations."""
    print("📊 Creating Performance Visualizations...\n")
    
    # Load GraphBit and LangChain results
    try:
        graphbit_results = load_json_results('graphbit_stress_50k.json')
        langchain_results = load_json_results('langchain_stress_50k.json')
    except FileNotFoundError as e:
        print(f"❌ Error: {e}")
        print("Make sure graphbit_stress_50k.json and langchain_stress_50k.json exist")
        return
    
    # Extract data
    graphbit_data = extract_test_data(graphbit_results, 'graphbit')
    langchain_data = extract_test_data(langchain_results, 'langchain')
    
    # Create charts
    create_chart1_total_time(graphbit_data, langchain_data)
    create_chart2_throughput(graphbit_data, langchain_data)
    create_chart3_speedup(graphbit_data, langchain_data)
    create_chart4_component_breakdown(graphbit_data, langchain_data)
    create_chart5_extended_capacity()
    
    print("\n✅ All visualizations created successfully!")

if __name__ == "__main__":
    main()

