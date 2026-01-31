use std::{collections::HashMap, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use crate::commands::Command;

mod commands;

struct KVStore {
    map: HashMap<String, String>,
}

impl KVStore {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    fn del(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }
}

async fn execute(cmd: Command, store: Arc<RwLock<KVStore>>) -> String {
    match cmd {
        Command::Set(key, val) => {
            let mut store = store.write().await;
            store.set(key, val);
            "OK".to_string()
        }
        Command::Get(key) => {
            let store = store.read().await;
            if let Some(val) = store.get(&key) {
                val
            } else {
                "(nil)".to_string()
            }
        }
        Command::Del(key) => {
            let mut store = store.write().await;

            if store.del(&key) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Command::Ping => "PONG".to_string(),
    }
}

// Async Implementation
//
async fn handle_stream(mut stream: TcpStream, store: Arc<RwLock<KVStore>>) {
    let mut buffer = [0; 1024];

    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let string = String::from_utf8(buffer[..n].to_vec()).expect("Invalid String");

        println!("Received: {}", string);

        let command = Command::parse(&string);
        match command {
            Ok(cmd) => {
                let value = execute(cmd, store.clone()).await;
                let _ = stream.write_all(value.as_bytes()).await;
                let _ = stream.write_all(b"\n").await;
            }
            Err(err) => {
                let _ = stream.write_all(err.as_bytes()).await;
                let _ = stream.write_all(b"\n").await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(KVStore::new()));

    let listener = TcpListener::bind("127.0.0.1:9736").await.unwrap();

    println!("Server started");

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let store = Arc::clone(&store);
        tokio::spawn(async move {
            handle_stream(stream, store).await;
        });
    }
}

// Multithreaded Implementation
//
// fn handle_stream(mut stream: TcpStream, store: Arc<Mutex<KVStore>>) {
//     let mut buffer = [0; 1024];
//     let n = stream.read(&mut buffer).unwrap();

//     let string = unsafe { String::from_utf8_unchecked(buffer[..n].to_vec()) };

//     println!("Received: {}", string);

//     let command = Command::parse(&string);
//     match command {
//         Ok(cmd) => {
//             let value = execute(cmd, store);
//             let _ = stream.write_all(value.as_bytes());
//             let _ = stream.write_all(b"\n");
//         }
//         Err(err) => {
//             let _ = stream.write_all(err.as_bytes());
//             let _ = stream.write_all(b"\n");
//         }
//     }
//     sleep(Duration::from_secs(3));
// }

// fn main() {
//     let store = Arc::new(Mutex::new(KVStore::new()));

//     let listener = TcpListener::bind("127.0.0.1:9736").unwrap();

//     println!("Server started");

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//         let store = Arc::clone(&store);
//         thread::spawn(move || handle_stream(stream, store));
//     }
// }

// Single Threaded Implementation
//
// fn main() {
//     let mut store = KVStore::new();

//     let listener = TcpListener::bind("127.0.0.1:9736").unwrap();

//     println!("Server started");

//     for stream in listener.incoming() {
//         let mut stream = stream.unwrap();
//         let mut buffer = [0; 1024];
//         let n = stream.read(&mut buffer).unwrap();

//         let string = unsafe { String::from_utf8_unchecked(buffer[..n].to_vec()) };

//         println!("Received: {}", string);

//         let command = Command::parse(&string);
//         match command {
//             Ok(cmd) => {
//                 let value = execute(cmd, &mut store);
//                 let _ = stream.write_all(value.as_bytes()).unwrap();
//                 let _ = stream.write_all(b"\n").unwrap();
//             }
//             Err(err) => {
//                 let _ = stream.write_all(err.as_bytes());
//                 let _ = stream.write_all(b"\n");
//             }
//         }

//         sleep(Duration::from_secs(3));
//     }
// }
