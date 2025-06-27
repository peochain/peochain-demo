use memory_monitor::{get_memory_monitor, record_allocation, record_deallocation, print_memory_stats};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_memory_monitor() {
    // Record some allocations
    record_allocation(1024); // 1 KB
    record_allocation(2048); // 2 KB
    
    // Check that allocations were recorded
    let monitor = get_memory_monitor();
    assert_eq!(monitor.get_allocated_memory(), 3072);
    assert_eq!(monitor.get_peak_memory(), 3072);
    assert_eq!(monitor.get_allocation_count(), 2);
    
    // Record a deallocation
    record_deallocation(1024);
    
    // Check that deallocation was recorded
    assert_eq!(monitor.get_allocated_memory(), 2048);
    assert_eq!(monitor.get_peak_memory(), 3072); // Peak remains unchanged
    
    // Record a larger allocation to update peak
    record_allocation(4096); // 4 KB
    assert_eq!(monitor.get_allocated_memory(), 6144);
    assert_eq!(monitor.get_peak_memory(), 6144);
    
    // Wait a bit to test uptime
    sleep(Duration::from_millis(100));
    assert!(monitor.get_uptime().as_millis() >= 100);
    
    // Print stats for visual inspection
    print_memory_stats();
    
    // Reset statistics
    monitor.reset();
    assert_eq!(monitor.get_allocated_memory(), 0);
    assert_eq!(monitor.get_peak_memory(), 0);
    assert_eq!(monitor.get_allocation_count(), 0);
}
