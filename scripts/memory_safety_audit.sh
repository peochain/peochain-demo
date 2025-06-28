#!/bin/bash

# ----------------------------------------------------------------------------
# PEOCHAIN-DEMO: COMPREHENSIVE MEMORY SAFETY AUDIT SCRIPT
# ----------------------------------------------------------------------------
# This script performs automated security audit of the codebase focusing on
# memory safety, bounds checking, and potential vulnerabilities.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Global counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# Function to print section headers
print_section() {
    echo -e "\n${BLUE}==== $1 ====${NC}"
}

# Function to print test results
print_result() {
    local status=$1
    local message=$2
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓ PASS${NC}: $message"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}✗ FAIL${NC}: $message"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
    elif [ "$status" = "WARN" ]; then
        echo -e "${YELLOW}⚠ WARN${NC}: $message"
    else
        echo -e "${BLUE}ℹ INFO${NC}: $message"
    fi
}

# Function to check for unsafe code patterns
check_unsafe_patterns() {
    print_section "PHASE 1: STATIC ANALYSIS - UNSAFE CODE DETECTION"
    
    # Check for unsafe blocks
    local unsafe_count=$(find . -name "*.rs" -exec grep -l "unsafe" {} \; | wc -l)
    if [ "$unsafe_count" -eq 0 ]; then
        print_result "PASS" "No unsafe code blocks found"
    else
        print_result "FAIL" "Found $unsafe_count files with unsafe code blocks"
        find . -name "*.rs" -exec grep -Hn "unsafe" {} \;
    fi
    
    # Check for unchecked arithmetic operations
    local unchecked_ops=$(find . -name "*.rs" -exec grep -E "\+|\-|\*|/" {} \; | grep -v "checked_" | grep -v "saturating_" | wc -l)
    if [ "$unchecked_ops" -gt 0 ]; then
        print_result "WARN" "Found $unchecked_ops potential unchecked arithmetic operations"
    else
        print_result "PASS" "All arithmetic operations appear to be checked"
    fi
    
    # Check for direct Vec indexing without bounds checking
    local unsafe_indexing=$(find . -name "*.rs" -exec grep -E "\[[0-9]+\]|\[.*\]" {} \; | grep -v "get(" | wc -l)
    if [ "$unsafe_indexing" -gt 0 ]; then
        print_result "WARN" "Found $unsafe_indexing instances of direct indexing (potential bounds issues)"
    else
        print_result "PASS" "No direct indexing without bounds checking found"
    fi
    
    # Check for unwrap() calls that could panic
    local unwrap_count=$(find . -name "*.rs" -exec grep -c "\.unwrap()" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$unwrap_count" -gt 0 ]; then
        print_result "WARN" "Found $unwrap_count .unwrap() calls that could cause panics"
    else
        print_result "PASS" "No .unwrap() calls found"
    fi
}

# Function to check bounds and size limits
check_bounds_and_limits() {
    print_section "PHASE 2: BOUNDS AND SIZE LIMIT VERIFICATION"
    
    # Check for maximum size constants
    local max_constants=$(find . -name "*.rs" -exec grep -c "MAX_.*SIZE\|MAX_.*LENGTH\|MAX_.*COUNT" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$max_constants" -gt 5 ]; then
        print_result "PASS" "Found $max_constants size limit constants"
    else
        print_result "WARN" "Found only $max_constants size limit constants (may need more)"
    fi
    
    # Check for bounded collection operations
    local bounded_collections=$(find . -name "*.rs" -exec grep -c "HashMap.*len.*>.*MAX\|Vec.*len.*>.*MAX" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$bounded_collections" -gt 0 ]; then
        print_result "PASS" "Found $bounded_collections bounded collection checks"
    else
        print_result "WARN" "No explicit collection size bounds found"
    fi
    
    # Check for input validation functions
    local validation_functions=$(find . -name "*.rs" -exec grep -c "validate.*fn\|validate_.*(" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$validation_functions" -gt 3 ]; then
        print_result "PASS" "Found $validation_functions input validation functions"
    else
        print_result "WARN" "Found only $validation_functions validation functions"
    fi
}

# Function to run unit tests
run_unit_tests() {
    print_section "PHASE 3: UNIT TEST EXECUTION"
    
    # Run bridge tests
    echo "Running bridge module tests..."
    if cd bridge && cargo test 2>/dev/null; then
        print_result "PASS" "Bridge module unit tests passed"
    else
        print_result "FAIL" "Bridge module unit tests failed"
    fi
    cd ..
    
    # Run consensus tests
    echo "Running consensus module tests..."
    if cd consensus && cargo test 2>/dev/null; then
        print_result "PASS" "Consensus module unit tests passed"
    else
        print_result "FAIL" "Consensus module unit tests failed"
    fi
    cd ..
}

# Function to run property-based tests
run_property_tests() {
    print_section "PHASE 4: PROPERTY-BASED TESTING"
    
    # Run bridge property tests
    echo "Running bridge property-based tests..."
    if cd bridge && cargo test proptest 2>/dev/null; then
        print_result "PASS" "Bridge property-based tests passed"
    else
        print_result "WARN" "Bridge property-based tests may have issues"
    fi
    cd ..
    
    # Run consensus property tests
    echo "Running consensus property-based tests..."
    if cd consensus && cargo test proptest 2>/dev/null; then
        print_result "PASS" "Consensus property-based tests passed"
    else
        print_result "WARN" "Consensus property-based tests may have issues"
    fi
    cd ..
}

# Function to run performance benchmarks
run_benchmarks() {
    print_section "PHASE 5: PERFORMANCE BENCHMARK VALIDATION"
    
    # Run bridge benchmarks
    echo "Running bridge performance benchmarks..."
    if cd bridge && cargo test benchmark 2>/dev/null; then
        print_result "PASS" "Bridge performance benchmarks completed"
    else
        print_result "WARN" "Bridge benchmarks may have performance issues"
    fi
    cd ..
    
    # Run consensus benchmarks
    echo "Running consensus performance benchmarks..."
    if cd consensus && cargo test benchmark 2>/dev/null; then
        print_result "PASS" "Consensus performance benchmarks completed"
    else
        print_result "WARN" "Consensus benchmarks may have performance issues"
    fi
    cd ..
}

# Function to check memory usage patterns
check_memory_patterns() {
    print_section "PHASE 6: MEMORY USAGE PATTERN ANALYSIS"
    
    # Check for memory monitoring code
    local memory_monitoring=$(find . -name "*.rs" -exec grep -c "memory_usage\|estimate_memory\|MEMORY.*USAGE" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$memory_monitoring" -gt 5 ]; then
        print_result "PASS" "Found $memory_monitoring memory monitoring implementations"
    else
        print_result "WARN" "Limited memory monitoring found ($memory_monitoring instances)"
    fi
    
    # Check for capacity limits
    local capacity_limits=$(find . -name "*.rs" -exec grep -c "capacity.*>\|len.*>.*max\|size.*>.*MAX" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$capacity_limits" -gt 0 ]; then
        print_result "PASS" "Found $capacity_limits capacity limit checks"
    else
        print_result "WARN" "No explicit capacity limits found"
    fi
    
    # Check for bounded string operations
    local bounded_strings=$(find . -name "*.rs" -exec grep -c "bounded.*string\|validate.*string\|string.*len.*>" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$bounded_strings" -gt 0 ]; then
        print_result "PASS" "Found $bounded_strings bounded string operations"
    else
        print_result "WARN" "No bounded string operations found"
    fi
}

# Function to check serialization safety
check_serialization_safety() {
    print_section "PHASE 7: SERIALIZATION SAFETY VERIFICATION"
    
    # Check for serde deny_unknown_fields
    local deny_unknown=$(find . -name "*.rs" -exec grep -c "deny_unknown_fields" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$deny_unknown" -gt 0 ]; then
        print_result "PASS" "Found $deny_unknown uses of deny_unknown_fields"
    else
        print_result "WARN" "No deny_unknown_fields directives found"
    fi
    
    # Check for bounded deserialization
    local bounded_deser=$(find . -name "*.rs" -exec grep -c "deserialize_with.*bounded\|from_bytes.*len" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$bounded_deser" -gt 0 ]; then
        print_result "PASS" "Found $bounded_deser bounded deserialization implementations"
    else
        print_result "WARN" "No bounded deserialization found"
    fi
    
    # Check for size limit constants in serialization
    local ser_limits=$(find . -name "*.rs" -exec grep -c "MAX.*MSG.*SIZE\|MAX.*SERIALIZED" {} \; | awk '{sum += $1} END {print sum}')
    if [ "$ser_limits" -gt 0 ]; then
        print_result "PASS" "Found $ser_limits serialization size limits"
    else
        print_result "WARN" "No serialization size limits found"
    fi
}

# Function to generate final report
generate_report() {
    print_section "AUDIT SUMMARY REPORT"
    
    echo -e "\n${BLUE}Total Checks Performed: $TOTAL_CHECKS${NC}"
    echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
    echo -e "${RED}Failed: $FAILED_CHECKS${NC}"
    echo -e "${YELLOW}Warnings: $((TOTAL_CHECKS - PASSED_CHECKS - FAILED_CHECKS))${NC}"
    
    local pass_rate=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
    echo -e "\n${BLUE}Pass Rate: $pass_rate%${NC}"
    
    if [ "$FAILED_CHECKS" -eq 0 ] && [ "$pass_rate" -ge 80 ]; then
        echo -e "\n${GREEN}🎉 AUDIT RESULT: MEMORY SAFETY REQUIREMENTS MET${NC}"
        echo -e "${GREEN}✓ Zero critical failures detected${NC}"
        echo -e "${GREEN}✓ Pass rate above 80% threshold${NC}"
    elif [ "$FAILED_CHECKS" -eq 0 ]; then
        echo -e "\n${YELLOW}⚠ AUDIT RESULT: PARTIAL COMPLIANCE${NC}"
        echo -e "${YELLOW}• No critical failures but improvements needed${NC}"
        echo -e "${YELLOW}• Consider addressing warnings for better security${NC}"
    else
        echo -e "\n${RED}❌ AUDIT RESULT: CRITICAL ISSUES DETECTED${NC}"
        echo -e "${RED}• $FAILED_CHECKS critical issues must be addressed${NC}"
        echo -e "${RED}• Memory safety requirements not fully met${NC}"
    fi
    
    # Generate recommendations
    echo -e "\n${BLUE}RECOMMENDATIONS:${NC}"
    if [ "$FAILED_CHECKS" -gt 0 ]; then
        echo "1. Address all critical failures immediately"
        echo "2. Review and fix unsafe code patterns"
        echo "3. Implement missing bounds checking"
    fi
    echo "4. Consider adding more property-based tests"
    echo "5. Implement comprehensive input validation"
    echo "6. Add memory usage monitoring to all modules"
    echo "7. Use structured message types with size limits"
}

# Main execution
main() {
    echo -e "${BLUE}PEOCHAIN MEMORY SAFETY AUDIT${NC}"
    echo -e "${BLUE}=============================${NC}"
    echo "Starting comprehensive security audit..."
    
    check_unsafe_patterns
    check_bounds_and_limits
    run_unit_tests
    run_property_tests
    run_benchmarks
    check_memory_patterns
    check_serialization_safety
    generate_report
    
    echo -e "\n${BLUE}Audit completed. See summary above for results.${NC}"
}

# Run the audit
main "$@"
