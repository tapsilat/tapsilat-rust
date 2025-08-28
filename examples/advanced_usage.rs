use tapsilat::hello;

fn main() {
    println!("Advanced Tapsilat Example");
    println!("========================");
    
    // Example 1: Using the hello function in different contexts
    let message = hello();
    println!("Basic message: {}", message);
    
    // Example 2: Using hello in a struct
    struct Greeter {
        name: String,
    }
    
    impl Greeter {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
        
        fn greet(&self) -> String {
            format!("{} from {}", hello(), self.name)
        }
    }
    
    let greeter = Greeter::new("Tapsilat SDK");
    println!("Greeter message: {}", greeter.greet());
    
    // Example 3: Using hello in a vector
    let messages: Vec<String> = (1..=5)
        .map(|_| hello())
        .collect();
    
    println!("Collection of messages: {:?}", messages);
    
    // Example 4: Conditional usage
    let should_greet = true;
    if should_greet {
        println!("Conditional greeting: {}", hello());
    }
    
    println!("\nAdvanced example completed!");
}
