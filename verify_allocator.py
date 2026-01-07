#!/usr/bin/env python3
"""
Quick script to verify memory allocator on macOS.
"""

import graphbit
import platform

def verify_allocator():
    """Verify the memory allocator is correctly configured."""
    
    # Get system info
    info = graphbit.get_system_info()
    
    print("=" * 60)
    print("GraphBit Memory Allocator Verification")
    print("=" * 60)
    
    # Platform info
    system = platform.system()
    machine = platform.machine()
    print(f"\n🖥️  Platform: {system} ({machine})")
    
    # Allocator info
    allocator = info['memory_allocator']
    verified = info['memory_allocator_verified']
    
    print(f"\n📊 Memory Allocator: {allocator}")
    print(f"✓  Runtime Verified: {verified}")
    
    # Verification result
    print("\n" + "=" * 60)
    if system == "Darwin" and allocator == "mimalloc" and verified:
        print("✅ SUCCESS: mimalloc is ACTIVE on macOS")
        print("   Expected performance: 6× throughput improvement")
        print("   Expected memory: 50% reduction")
        return True
    elif allocator == "mimalloc" and verified:
        print(f"✅ SUCCESS: mimalloc is ACTIVE on {system}")
        return True
    else:
        print(f"⚠️  WARNING: Expected mimalloc on macOS, got {allocator}")
        print(f"   Verified: {verified}")
        return False
    
if __name__ == "__main__":
    success = verify_allocator()
    exit(0 if success else 1)
