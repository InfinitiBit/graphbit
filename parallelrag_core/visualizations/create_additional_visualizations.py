#!/usr/bin/env python3
"""
Create additional visualization charts for GraphBit performance analysis.
"""

import json
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from typing import List, Dict, Any

# Set style
plt.style.use('seaborn-v0_8-darkgrid')

def load_json_results(filename: str) -> List[Dict[str, Any]]:
    """Load JSON results from file."""
    with open(filename, 'r') as f:
        return json.load(f)

def create_chart_worker_optimization(output_file='chart_worker_optimization.png'):
    """Chart: Worker Count Optimization (Throughput vs Workers)."""
    print(f"Creating {output_file}...")
    
    # Load worker optimization results
    data = load_json_results('worker_optimization_results.json')
    results = data['results']
    
    workers = [r['num_workers'] for r in results]
    throughputs = [r['throughput_docs_per_sec'] for r in results]
    
    fig, ax = plt.subplots(figsize=(12, 7))
    
    # Plot throughput
    ax.plot(workers, throughputs, marker='o', linewidth=2.5, markersize=10, 
            color='#2E86AB', label='Throughput')
    
    # Highlight optimal point
    optimal_idx = throughputs.index(max(throughputs))
    ax.plot(workers[optimal_idx], throughputs[optimal_idx], marker='*', 
            markersize=20, color='#A23B72', label='Optimal Configuration')
    
    # Add value labels
    for i, (w, t) in enumerate(zip(workers, throughputs)):
        ax.annotate(f'{t:.0f} docs/sec', 
                   xy=(w, t), 
                   xytext=(0, 10), 
                   textcoords='offset points',
                   ha='center',
                   fontsize=9,
                   bbox=dict(boxstyle='round,pad=0.3', facecolor='yellow', alpha=0.3))
    
    ax.set_xlabel('Number of Workers', fontsize=12, fontweight='bold')
    ax.set_ylabel('Throughput (docs/sec)', fontsize=12, fontweight='bold')
    ax.set_title('GraphBit Worker Count Optimization\n5,000 Documents (200 words each)', 
                 fontsize=14, fontweight='bold', pad=20)
    ax.legend(fontsize=11)
    ax.grid(True, alpha=0.3)
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart_document_size_impact(output_file='chart_document_size_impact.png'):
    """Chart: Document Size Impact on Performance."""
    print(f"Creating {output_file}...")
    
    # Load variable size results
    results_100w = load_json_results('graphbit_variable_size_100w.json')
    results_2000w = load_json_results('graphbit_variable_size_2000w.json')
    results_10000w = load_json_results('graphbit_variable_size_10000w.json')
    
    # Extract 5000 doc results
    def get_5k_result(results):
        for test in results:
            if test.get('graphbit') and test['graphbit']['num_documents'] == 5000:
                return test['graphbit']
        return None
    
    result_100w = get_5k_result(results_100w)
    result_2000w = get_5k_result(results_2000w)
    result_10000w = get_5k_result(results_10000w)
    
    # Also get 200 words from existing data
    results_200w = load_json_results('graphbit_stress_50k.json')
    result_200w = None
    for test in results_200w:
        if test.get('graphbit') and test['graphbit']['num_documents'] == 5000:
            result_200w = test['graphbit']
            break
    
    word_counts = [100, 200, 2000, 10000]
    doc_throughputs = [
        result_100w['throughput_docs_per_sec'],
        result_200w['throughput_docs_per_sec'],
        result_2000w['throughput_docs_per_sec'],
        result_10000w['throughput_docs_per_sec']
    ]
    chunk_throughputs = [
        result_100w['chunks_created'] / result_100w['total_time'],
        result_200w['chunks_created'] / result_200w['total_time'],
        result_2000w['chunks_created'] / result_2000w['total_time'],
        result_10000w['chunks_created'] / result_10000w['total_time']
    ]
    
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7))
    
    # Document throughput
    ax1.bar(range(len(word_counts)), doc_throughputs, color='#2E86AB', alpha=0.8)
    ax1.set_xlabel('Words per Document', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Document Throughput (docs/sec)', fontsize=12, fontweight='bold')
    ax1.set_title('Document Throughput vs Document Size', fontsize=13, fontweight='bold')
    ax1.set_xticks(range(len(word_counts)))
    ax1.set_xticklabels([f'{w:,}' for w in word_counts])
    ax1.grid(True, alpha=0.3, axis='y')
    
    # Add value labels
    for i, v in enumerate(doc_throughputs):
        ax1.text(i, v + 30, f'{v:.0f}', ha='center', fontsize=10, fontweight='bold')
    
    # Chunk throughput
    ax2.bar(range(len(word_counts)), chunk_throughputs, color='#A23B72', alpha=0.8)
    ax2.set_xlabel('Words per Document', fontsize=12, fontweight='bold')
    ax2.set_ylabel('Chunk Throughput (chunks/sec)', fontsize=12, fontweight='bold')
    ax2.set_title('Chunk Throughput vs Document Size', fontsize=13, fontweight='bold')
    ax2.set_xticks(range(len(word_counts)))
    ax2.set_xticklabels([f'{w:,}' for w in word_counts])
    ax2.grid(True, alpha=0.3, axis='y')
    
    # Add value labels
    for i, v in enumerate(chunk_throughputs):
        ax2.text(i, v + 1000, f'{v:.0f}', ha='center', fontsize=10, fontweight='bold')
    
    plt.suptitle('GraphBit Performance vs Document Size\n5,000 Documents', 
                 fontsize=14, fontweight='bold', y=1.02)
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart_cost_comparison(output_file='chart_cost_comparison.png'):
    """Chart: Cost Comparison GraphBit vs LangChain."""
    print(f"Creating {output_file}...")
    
    # Load comparison results
    graphbit_results = load_json_results('graphbit_stress_50k.json')
    langchain_results = load_json_results('langchain_stress_50k.json')
    
    # Extract data for different document counts
    doc_counts = [100, 1000, 5000, 10000, 25000, 50000]
    graphbit_costs = []
    langchain_costs = []
    
    # AWS c5.4xlarge cost: $0.68/hour
    hourly_cost = 0.68
    
    for count in doc_counts:
        # Find GraphBit result
        for test in graphbit_results:
            if test.get('graphbit') and test['graphbit']['num_documents'] == count:
                time_hours = test['graphbit']['total_time'] / 3600
                graphbit_costs.append(time_hours * hourly_cost)
                break
        
        # Find LangChain result
        for test in langchain_results:
            if test.get('langchain') and test['langchain']['num_documents'] == count:
                time_hours = test['langchain']['total_time'] / 3600
                langchain_costs.append(time_hours * hourly_cost)
                break
    
    x = range(len(doc_counts))
    width = 0.35
    
    fig, ax = plt.subplots(figsize=(14, 8))
    
    bars1 = ax.bar([i - width/2 for i in x], graphbit_costs, width, 
                   label='GraphBit', color='#2E86AB', alpha=0.8)
    bars2 = ax.bar([i + width/2 for i in x], langchain_costs, width,
                   label='LangChain', color='#F18F01', alpha=0.8)
    
    ax.set_xlabel('Number of Documents', fontsize=12, fontweight='bold')
    ax.set_ylabel('Cost (USD)', fontsize=12, fontweight='bold')
    ax.set_title('Processing Cost Comparison: GraphBit vs LangChain\nAWS c5.4xlarge ($0.68/hour)', 
                 fontsize=14, fontweight='bold', pad=20)
    ax.set_xticks(x)
    ax.set_xticklabels([f'{c:,}' for c in doc_counts])
    ax.legend(fontsize=11)
    ax.grid(True, alpha=0.3, axis='y')
    
    # Add value labels and savings
    for i, (g_cost, l_cost) in enumerate(zip(graphbit_costs, langchain_costs)):
        ax.text(i - width/2, g_cost + 0.001, f'${g_cost:.3f}', 
               ha='center', va='bottom', fontsize=9)
        ax.text(i + width/2, l_cost + 0.001, f'${l_cost:.3f}', 
               ha='center', va='bottom', fontsize=9)
        
        # Add savings percentage
        savings = ((l_cost - g_cost) / l_cost) * 100
        ax.text(i, max(g_cost, l_cost) + 0.01, f'{savings:.0f}% savings', 
               ha='center', fontsize=9, fontweight='bold', color='green',
               bbox=dict(boxstyle='round,pad=0.3', facecolor='lightgreen', alpha=0.5))
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def create_chart_scaling_efficiency(output_file='chart_scaling_efficiency.png'):
    """Chart: GraphBit Scaling Efficiency (100 to 500K documents)."""
    print(f"Creating {output_file}...")

    # Load all GraphBit results
    results_50k = load_json_results('graphbit_stress_50k.json')
    results_100k = load_json_results('graphbit_max_capacity_100k.json')
    results_250k = load_json_results('graphbit_max_capacity_250k.json')
    results_500k = load_json_results('graphbit_max_capacity_500k.json')

    # Combine all results
    all_results = []
    for results in [results_50k, results_100k, results_250k, results_500k]:
        for test in results:
            if test.get('graphbit'):
                all_results.append(test['graphbit'])

    # Sort by document count
    all_results.sort(key=lambda x: x['num_documents'])

    # Remove duplicates (keep first occurrence)
    seen = set()
    unique_results = []
    for r in all_results:
        if r['num_documents'] not in seen:
            seen.add(r['num_documents'])
            unique_results.append(r)

    doc_counts = [r['num_documents'] for r in unique_results]
    throughputs = [r['throughput_docs_per_sec'] for r in unique_results]
    total_times = [r['total_time'] for r in unique_results]

    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(14, 10))

    # Throughput vs document count
    ax1.plot(doc_counts, throughputs, marker='o', linewidth=2.5, markersize=8,
            color='#2E86AB', label='Throughput')
    ax1.axhline(y=900, color='red', linestyle='--', linewidth=1.5,
               label='Stable Throughput (~900 docs/sec)', alpha=0.7)
    ax1.set_xlabel('Number of Documents', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Throughput (docs/sec)', fontsize=12, fontweight='bold')
    ax1.set_title('GraphBit Throughput Scaling (100 to 500,000 Documents)',
                 fontsize=13, fontweight='bold')
    ax1.set_xscale('log')
    ax1.legend(fontsize=10)
    ax1.grid(True, alpha=0.3)

    # Add annotations for key points
    peak_idx = throughputs.index(max(throughputs))
    ax1.annotate(f'Peak: {throughputs[peak_idx]:.0f} docs/sec\n@ {doc_counts[peak_idx]:,} docs',
                xy=(doc_counts[peak_idx], throughputs[peak_idx]),
                xytext=(doc_counts[peak_idx] * 0.3, throughputs[peak_idx] + 200),
                arrowprops=dict(arrowstyle='->', color='red', lw=2),
                fontsize=10, fontweight='bold',
                bbox=dict(boxstyle='round,pad=0.5', facecolor='yellow', alpha=0.7))

    # Total time vs document count
    ax2.plot(doc_counts, total_times, marker='s', linewidth=2.5, markersize=8,
            color='#A23B72', label='Total Processing Time')
    ax2.set_xlabel('Number of Documents', fontsize=12, fontweight='bold')
    ax2.set_ylabel('Total Time (seconds)', fontsize=12, fontweight='bold')
    ax2.set_title('GraphBit Processing Time Scaling', fontsize=13, fontweight='bold')
    ax2.set_xscale('log')
    ax2.set_yscale('log')
    ax2.legend(fontsize=10)
    ax2.grid(True, alpha=0.3)

    # Add linear scaling reference line
    if len(doc_counts) >= 2:
        # Use first two points to establish baseline
        x1, y1 = doc_counts[0], total_times[0]
        x2 = doc_counts[-1]
        y2_linear = y1 * (x2 / x1)  # Linear scaling
        ax2.plot([x1, x2], [y1, y2_linear], 'r--', linewidth=1.5,
                label='Linear Scaling (ideal)', alpha=0.7)
        ax2.legend(fontsize=10)

    plt.suptitle('GraphBit Scaling Efficiency Analysis',
                fontsize=14, fontweight='bold', y=0.995)
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    plt.close()
    print(f"✅ Created: {output_file}")

def main():
    """Create all additional visualizations."""
    print("📊 Creating Additional Performance Visualizations...\n")

    create_chart_worker_optimization()
    create_chart_document_size_impact()
    create_chart_cost_comparison()
    create_chart_scaling_efficiency()

    print("\n✅ All additional visualizations created successfully!")

if __name__ == "__main__":
    main()

