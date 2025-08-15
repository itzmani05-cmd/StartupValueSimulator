// Simple test file to check if Rust analyzer is working
fn main() {
    println!("Hello, Rust!");
    
    let x = 42;
    let y = x * 2;
    
    println!("x = {}, y = {}", x, y);
    
    // Test some basic Rust features
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Sum of numbers: {}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
    }
}
