"""
Create resource utilization charts from GraphBit performance test results.

This script generates:
1. Memory Usage Chart - Memory consumption across document scales
2. CPU Utilization Chart - CPU usage patterns
3. Resource Efficiency Chart - Throughput per GB memory

Author: GraphBit Performance Engineering Team
Date: November 17, 2025
"""

import json
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from pathlib import Path

# Use seaborn style for consistency
plt.style.use('seaborn-v0_8-darkgrid')

def load_json_results(filename):
    """Load JSON results file."""
    try:
        with open(filename, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"Warning: {filename} not found")
        return []

def extract_graphbit_data(results, doc_count):
    """Extract GraphBit data for specific document count."""
    for test in results:
        if test.get('num_documents') == doc_count and test.get('graphbit'):
            return test['graphbit']
    return None

def create_chart_memory_usage(output_file='chart_memory_usage.png'):
    """Chart: Memory Usage Across Document Scales."""
    print("Creating Memory Usage Chart...")
    
    # Load all result files
    files_and_counts = [
        ('graphbit_stress_50k.json', [100, 500, 1000, 5000, 10000, 25000, 50000]),
        ('graphbit_max_capacity_100k.json', [100000]),
        ('graphbit_max_capacity_250k.json', [250000]),
        ('graphbit_max_capacity_500k.json', [500000])
    ]
    
    doc_counts = []
    memory_gb = []
    memory_per_doc_mb = []
    
    for filename, counts in files_and_counts:
        results = load_json_results(filename)
        for count in counts:
            data = extract_graphbit_data(results, count)
            if data:
                doc_counts.append(count)
                mem_mb = data.get('peak_memory_mb', 0)
                memory_gb.append(mem_mb / 1024)  # Convert to GB
                memory_per_doc_mb.append(mem_mb / count if count > 0 else 0)
    
    if not doc_counts:
        print("No memory data found. Skipping memory chart.")
        return
    
    # Create figure with two subplots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    
    # Subplot 1: Total Memory Usage
    ax1.plot(doc_counts, memory_gb, marker='o', linewidth=2, markersize=8, color='#2E86AB')
    ax1.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Peak Memory Usage (GB)', fontsize=12, fontweight='bold')
    ax1.set_title('GraphBit Memory Usage Across Scales', fontsize=14, fontweight='bold')
    ax1.set_xscale('log')
    ax1.grid(True, alpha=0.3)
    
    # Add value labels
    for x, y in zip(doc_counts, memory_gb):
        ax1.annotate(f'{y:.2f} GB', (x, y), textcoords="offset points", 
                    xytext=(0,10), ha='center', fontsize=9)
    
    # Add 18 GB threshold line
    ax1.axhline(y=18, color='red', linestyle='--', linewidth=2, alpha=0.7, label='Available Memory (18 GB)')
    ax1.legend(fontsize=10)
    
    # Subplot 2: Memory Per Document
    ax2.plot(doc_counts, memory_per_doc_mb, marker='s', linewidth=2, markersize=8, color='#A23B72')
    ax2.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax2.set_ylabel('Memory Per Document (MB)', fontsize=12, fontweight='bold')
    ax2.set_title('Memory Efficiency (Lower is Better)', fontsize=14, fontweight='bold')
    ax2.set_xscale('log')
    ax2.grid(True, alpha=0.3)
    
    # Add value labels for key points
    for i, (x, y) in enumerate(zip(doc_counts, memory_per_doc_mb)):
        if i % 2 == 0 or i == len(doc_counts) - 1:  # Label every other point
            ax2.annotate(f'{y:.4f} MB', (x, y), textcoords="offset points", 
                        xytext=(0,10), ha='center', fontsize=9)
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"✅ Memory Usage Chart saved to {output_file}")
    plt.close()

def create_chart_cpu_utilization(output_file='chart_cpu_utilization.png'):
    """Chart: CPU Utilization Patterns."""
    print("Creating CPU Utilization Chart...")

    # Load results from graphbit_stress_50k.json which has CPU data
    results = load_json_results('graphbit_stress_50k.json')

    if not results:
        print("No results data found. Skipping CPU chart.")
        return

    doc_counts = []
    avg_cpu = []
    peak_cpu = []

    # Extract CPU data for different document counts
    for count in [100, 500, 1000, 5000, 10000, 25000, 50000]:
        data = extract_graphbit_data(results, count)
        if data:
            doc_counts.append(count)
            avg_cpu.append(data.get('avg_cpu_percent', 0))
            peak_cpu.append(data.get('peak_cpu_percent', 0))

    if not doc_counts:
        print("No CPU data found. Skipping CPU chart.")
        return
    
    # Create figure
    fig, ax = plt.subplots(figsize=(10, 6))

    # Plot both average and peak CPU
    ax.plot(doc_counts, avg_cpu, marker='o', linewidth=2, markersize=8,
            color='#2E86AB', label='Average CPU %')
    ax.plot(doc_counts, peak_cpu, marker='s', linewidth=2, markersize=8,
            color='#F18F01', label='Peak CPU %', linestyle='--')

    ax.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax.set_ylabel('CPU Utilization (%)', fontsize=12, fontweight='bold')
    ax.set_title('GraphBit CPU Utilization Across Scales (20 workers)', fontsize=14, fontweight='bold')
    ax.set_xscale('log')
    ax.grid(True, alpha=0.3)
    ax.legend(fontsize=11)

    # Add annotation about multi-core utilization
    if len(doc_counts) > 0:
        mid_idx = len(doc_counts) // 2
        ax.annotate(f'Excellent multi-core utilization\n{avg_cpu[mid_idx]:.1f}% avg CPU',
                   (doc_counts[mid_idx], avg_cpu[mid_idx]),
                   textcoords="offset points", xytext=(30,30), ha='left',
                   fontsize=10, fontweight='bold',
                   bbox=dict(boxstyle='round,pad=0.5', facecolor='lightgreen', alpha=0.7),
                   arrowprops=dict(arrowstyle='->', connectionstyle='arc3,rad=0.3', lw=2))
    
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"✅ CPU Utilization Chart saved to {output_file}")
    plt.close()

def create_chart_resource_efficiency(output_file='chart_resource_efficiency.png'):
    """Chart: Resource Efficiency (Throughput per GB Memory)."""
    print("Creating Resource Efficiency Chart...")

    # Load all result files
    files_and_counts = [
        ('graphbit_stress_50k.json', [100, 500, 1000, 5000, 10000, 25000, 50000]),
        ('graphbit_max_capacity_100k.json', [100000]),
        ('graphbit_max_capacity_250k.json', [250000]),
        ('graphbit_max_capacity_500k.json', [500000])
    ]

    doc_counts = []
    efficiency = []  # docs/sec per GB memory

    for filename, counts in files_and_counts:
        results = load_json_results(filename)
        for count in counts:
            data = extract_graphbit_data(results, count)
            if data:
                throughput = data.get('throughput_docs_per_sec', 0)
                mem_gb = data.get('peak_memory_mb', 0) / 1024
                if mem_gb > 0:
                    doc_counts.append(count)
                    efficiency.append(throughput / mem_gb)

    if not doc_counts:
        print("No efficiency data found. Skipping efficiency chart.")
        return

    # Create figure
    fig, ax = plt.subplots(figsize=(10, 6))

    ax.plot(doc_counts, efficiency, marker='D', linewidth=2, markersize=8, color='#06A77D')
    ax.set_xlabel('Document Count', fontsize=12, fontweight='bold')
    ax.set_ylabel('Throughput per GB Memory (docs/sec/GB)', fontsize=12, fontweight='bold')
    ax.set_title('GraphBit Resource Efficiency', fontsize=14, fontweight='bold')
    ax.set_xscale('log')
    ax.grid(True, alpha=0.3)

    # Add value labels for key points
    for i, (x, y) in enumerate(zip(doc_counts, efficiency)):
        if i == 0 or i == len(doc_counts) - 1 or i == len(doc_counts) // 2:
            ax.annotate(f'{y:.0f}', (x, y), textcoords="offset points",
                       xytext=(0,10), ha='center', fontsize=9, fontweight='bold')

    # Add annotation
    ax.text(0.5, 0.95, 'Higher values = Better resource utilization',
            transform=ax.transAxes, fontsize=11, verticalalignment='top',
            horizontalalignment='center', bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.5))

    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"✅ Resource Efficiency Chart saved to {output_file}")
    plt.close()

if __name__ == '__main__':
    print("=" * 60)
    print("GraphBit Resource Utilization Chart Generator")
    print("=" * 60)
    print()

    # Create all charts
    create_chart_memory_usage()
    print()
    create_chart_cpu_utilization()
    print()
    create_chart_resource_efficiency()

    print()
    print("=" * 60)
    print("✅ All resource charts created successfully!")
    print("=" * 60)
    print()
    print("Generated files:")
    print("  - chart_memory_usage.png")
    print("  - chart_cpu_utilization.png")
    print("  - chart_resource_efficiency.png")
    print()


