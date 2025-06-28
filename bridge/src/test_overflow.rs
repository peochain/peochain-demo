fn main() {
    println!("u64::MAX = {}", u64::MAX);
    println!("18446744073709551257 > u64::MAX: {}", 18446744073709551257u64 > u64::MAX);
    println!("Value in failing test: 18446744073709551257");
    println!("18446744073709551257 + 1 = {}", 18446744073709551257u64.wrapping_add(1));
    
    // Test if the value can actually overflow
    let large_val = 18446744073709551257u64;
    println!("large_val: {}", large_val);
    match large_val.checked_add(1) {
        Some(val) => println!("Result: {}", val),
        None => println!("Overflow detected!"),
    }
}
