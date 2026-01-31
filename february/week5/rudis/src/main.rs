use redis::Commands;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: rudis <COMMAND> <KEY> [VALUE]");
        eprintln!("Commands:");
        eprintln!("  SET <key> <value>  - Set a key to a value");
        eprintln!("  GET <key>          - Get the value of a key");
        process::exit(1);
    }

    let command = args[1].to_uppercase();
    let key = &args[2];

    // Connect to Redis
    let client = match redis::Client::open("redis://127.0.0.1:6379/") {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to Redis: {}", e);
            process::exit(1);
        }
    };

    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(e) => {
            eprintln!("Failed to get Redis connection: {}", e);
            process::exit(1);
        }
    };

    match command.as_str() {
        "SET" => {
            if args.len() < 4 {
                eprintln!("SET command requires a value");
                eprintln!("Usage: rudis SET <key> <value>");
                process::exit(1);
            }
            let value = &args[3];

            match con.set::<_, _, ()>(key, value) {
                Ok(_) => println!("OK"),
                Err(e) => {
                    eprintln!("Failed to set key: {}", e);
                    process::exit(1);
                }
            }
        }
        "GET" => match con.get::<_, Option<String>>(key) {
            Ok(Some(value)) => println!("{}", value),
            Ok(None) => println!("(nil)"),
            Err(e) => {
                eprintln!("Failed to get key: {}", e);
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Supported commands: SET, GET");
            process::exit(1);
        }
    }
}
