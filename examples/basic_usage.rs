use tapsilat::hello;

fn main() {
    println!("Running Tapsilat example...");
    
    // Call the hello function from the tapsilat library
    let greeting = hello();
    println!("Greeting from tapsilat: {}", greeting);
    
    // Show some usage scenarios
    println!("\nExample usage scenarios:");
    println!("1. Simple greeting: {}", hello());
    println!("2. Formatted greeting: Hello from {}!", "Tapsilat");
    println!("3. Multiple calls:");
    
    for i in 1..=3 {
        println!("   Call {}: {}", i, hello());
    }
    
    println!("\nExample completed successfully!");
}
