//! Memory monitoring utilities for PeoChain modules
//! 
//! This module provides APIs for tracking and reporting memory usage across
//! all PeoChain components.

use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Global memory usage statistics
static GLOBAL_MEMORY_MONITOR: MemoryMonitor = MemoryMonitor::new();

/// Monitor for tracking memory usage across the system
pub struct MemoryMonitor {
    /// Total allocated memory in bytes
    allocated_memory: AtomicUsize,
    /// Maximum memory usage observed
    peak_memory: AtomicUsize,
    /// Number of memory allocation operations
    allocation_count: AtomicU64,
    /// When the monitor was started
    start_time: Instant,
}

impl MemoryMonitor {
    /// Creates a new memory monitor
    pub const fn new() -> Self {
        Self {
            allocated_memory: AtomicUsize::new(0),
            peak_memory: AtomicUsize::new(0),
            allocation_count: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }
    
    /// Records memory allocation
    pub fn record_allocation(&self, bytes: usize) {
        // Update allocation count
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
        
        // Update total allocated memory
        let new_total = self.allocated_memory.fetch_add(bytes, Ordering::Relaxed) + bytes;
        
        // Update peak memory if needed
        let mut current_peak = self.peak_memory.load(Ordering::Relaxed);
        while new_total > current_peak {
            match self.peak_memory.compare_exchange(
                current_peak,
                new_total,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => current_peak = actual,
            }
        }
    }
    
    /// Records memory deallocation
    pub fn record_deallocation(&self, bytes: usize) {
        self.allocated_memory.fetch_sub(bytes, Ordering::Relaxed);
    }
    
    /// Gets the currently allocated memory
    pub fn get_allocated_memory(&self) -> usize {
        self.allocated_memory.load(Ordering::Relaxed)
    }
    
    /// Gets the peak memory usage
    pub fn get_peak_memory(&self) -> usize {
        self.peak_memory.load(Ordering::Relaxed)
    }
    
    /// Gets the number of allocation operations
    pub fn get_allocation_count(&self) -> u64 {
        self.allocation_count.load(Ordering::Relaxed)
    }
    
    /// Gets the monitor's uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Prints memory usage statistics
    pub fn print_stats(&self) {
        let allocated = self.get_allocated_memory();
        let peak = self.get_peak_memory();
        let allocations = self.get_allocation_count();
        let uptime = self.get_uptime();
        
        println!("===== MEMORY USAGE STATISTICS =====");
        println!("Current memory usage: {} KB ({} MB)", allocated / 1024, allocated / (1024 * 1024));
        println!("Peak memory usage: {} KB ({} MB)", peak / 1024, peak / (1024 * 1024));
        println!("Total allocations: {}", allocations);
        println!("Uptime: {:.2} hours", uptime.as_secs_f64() / 3600.0);
        println!("==================================");
    }
    
    /// Resets the memory statistics (except uptime)
    pub fn reset(&self) {
        self.allocated_memory.store(0, Ordering::Relaxed);
        self.peak_memory.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
    }
}

/// Gets the global memory monitor
pub fn get_memory_monitor() -> &'static MemoryMonitor {
    &GLOBAL_MEMORY_MONITOR
}

/// Records memory allocation in the global monitor
pub fn record_allocation(bytes: usize) {
    GLOBAL_MEMORY_MONITOR.record_allocation(bytes);
}

/// Records memory deallocation in the global monitor
pub fn record_deallocation(bytes: usize) {
    GLOBAL_MEMORY_MONITOR.record_deallocation(bytes);
}

/// Prints global memory statistics
pub fn print_memory_stats() {
    GLOBAL_MEMORY_MONITOR.print_stats();
}
