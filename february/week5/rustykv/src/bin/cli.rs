use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9736").await?;
    println!("Connected to rustykv at 127.0.0.1:9736");

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buffer = [0; 1024];

    loop {
        print!("rustykv> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).await?;
        if bytes_read == 0 {
            break; // EOF
        }

        let input_trimmed = input.trim();
        if input_trimmed.is_empty() {
            continue;
        }

        if input_trimmed.eq_ignore_ascii_case("exit") || input_trimmed.eq_ignore_ascii_case("quit")
        {
            println!("Bye!");
            break;
        }

        if let Err(e) = stream.write_all(input_trimmed.as_bytes()).await {
            eprintln!("Failed to write to socket: {}", e);
            break;
        }

        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Server disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        };

        let response = String::from_utf8_lossy(&buffer[..n]);
        print!("{}", response);
    }

    Ok(())
}
