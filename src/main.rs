use tapsilat::{Config, TapsilatClient};

fn main() {
    let config = Config::new("your-api-key-here").with_timeout(30);

    match TapsilatClient::new(config) {
        Ok(client) => {
            println!("Tapsilat SDK initialized successfully!");
            println!(
                "Client ready to use: {}",
                std::any::type_name_of_val(&client)
            );
        }
        Err(e) => {
            eprintln!("Failed to initialize Tapsilat SDK: {}", e);
        }
    }
}
