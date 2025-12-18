use std::env;
use tapsilat::TapsilatClient;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Client
    let api_key = env::var("TAPSILAT_API_KEY").expect("TAPSILAT_API_KEY must be set");
    let client = TapsilatClient::from_api_key(api_key)?;

    println!("🚀 Starting SDK Verification...");

    // 2. Fetch Organization Settings
    println!("\n--- Organization Settings ---");
    match client.get_organization_settings() {
        Ok(settings) => println!("Settings: {:?}", settings),
        Err(e) => eprintln!("Error fetching settings: {}", e),
    }

    // 3. List Orders
    println!("\n--- Listing Orders ---");
    match client.get_order_list(1, 5, None) {
        Ok(orders) => println!("Orders (First 5): {:?}", orders),
        Err(e) => eprintln!("Error listing orders: {}", e),
    }

    // 4. List Subscriptions (New Feature)
    println!("\n--- Listing Subscriptions ---");
    match client.list_subscriptions(1, 5) {
        Ok(subs) => println!("Subscriptions (First 5): {:?}", subs),
        Err(e) => eprintln!("Error listing subscriptions: {}", e),
    }

    // 5. Check Health
    println!("\n--- API Health ---");
    match client.health_check() {
        Ok(health) => println!("Health: {:?}", health),
        Err(e) => eprintln!("Error checking health: {}", e),
    }

    Ok(())
}