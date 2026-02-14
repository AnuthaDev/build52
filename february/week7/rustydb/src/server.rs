use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    parser::Statement,
    storage::{Database, Row},
};

pub struct Server {
    listener: TcpListener,
    db: Arc<Mutex<Database>>,
}

impl Server {
    /// Bind to the given address (e.g. "127.0.0.1:7878") and return a Server.
    pub fn bind(addr: &str, db: Database) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        println!("RustyDB listening on {}", addr);
        Ok(Self {
            listener,
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Accept connections in a loop. Each client gets its own thread.
    pub fn run(self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let db = Arc::clone(&self.db);
                    let peer = stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    println!("[+] Client connected: {}", peer);
                    thread::spawn(move || {
                        handle_client(stream, db, &peer);
                        println!("[-] Client disconnected: {}", peer);
                    });
                }
                Err(e) => eprintln!("Accept error: {}", e),
            }
        }
    }
}

/// Handle one client connection for its entire lifetime.
fn handle_client(stream: TcpStream, db: Arc<Mutex<Database>>, peer: &str) {
    // Clone the stream so we can have separate read/write ends.
    let write_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[{}] Failed to clone stream: {}", peer, e);
            return;
        }
    };

    let mut writer = std::io::BufWriter::new(write_stream);
    let reader = BufReader::new(stream);

    // Greet the client
    let _ = writeln!(writer, "RustyDB ready. Type SQL or 'quit' to exit.");
    let _ = writer.flush();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // client disconnected
        };

        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.to_lowercase() == "quit" || trimmed.to_lowercase() == "exit" {
            let _ = writeln!(writer, "BYE");
            let _ = writer.flush();
            break;
        }

        let response = execute_statement(&trimmed, &db);
        let _ = writeln!(writer, "{}", response);
        let _ = writer.flush();
    }
}

/// Parse and execute a SQL statement, returning the response as a string.
fn execute_statement(input: &str, db: &Arc<Mutex<Database>>) -> String {
    match Statement::parse(input) {
        Err(e) => format!("ERROR: {}", e),
        Ok(statement) => match statement {
            Statement::Insert {
                table,
                columns: _,
                values,
            } => {
                let mut db = db.lock().unwrap();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    db.insert_into_table(&table, Row::new(values.clone()));
                })) {
                    Ok(_) => format!("OK: Inserted 1 row into '{}'.", table),
                    Err(_) => format!(
                        "ERROR: Insert into '{}' failed (wrong number of values or table not found).",
                        table
                    ),
                }
            }

            Statement::Select {
                table,
                columns,
                condition,
            } => {
                let db = db.lock().unwrap();

                // Get table metadata
                let table_meta = match db.get_table(&table) {
                    Some(t) => t,
                    None => return format!("ERROR: Table '{}' not found.", table),
                };

                let all_cols = &table_meta.columns;

                // Resolve column indices to display
                let col_indices: Vec<usize> = if columns.len() == 1 && columns[0] == "*" {
                    (0..all_cols.len()).collect()
                } else {
                    columns
                        .iter()
                        .filter_map(|name| all_cols.iter().position(|c| c == name))
                        .collect()
                };

                if col_indices.is_empty() {
                    return "ERROR: No valid columns selected.".to_string();
                }

                // Build header
                let header: Vec<&str> = col_indices.iter().map(|&i| all_cols[i].as_str()).collect();
                let header_str = header.join(" | ");
                let separator = "-".repeat(header_str.len());

                // Fetch rows
                let rows: Vec<Vec<String>> = match &condition {
                    None => db
                        .select_all(&table)
                        .iter()
                        .map(|r| r.get_inner_vec().clone())
                        .collect(),
                    Some(cond) => db
                        .select_where(&table, &cond.column, &cond.value)
                        .into_iter()
                        .map(|r| r.get_inner_vec().clone())
                        .collect(),
                };

                let mut output = format!("{}\n{}", header_str, separator);

                if rows.is_empty() {
                    output.push_str("\n(no rows)");
                } else {
                    for row in &rows {
                        let display: Vec<&str> =
                            col_indices.iter().map(|&i| row[i].as_str()).collect();
                        output.push('\n');
                        output.push_str(&display.join(" | "));
                    }
                    let count = rows.len();
                    output.push_str(&format!(
                        "\n({} row{})",
                        count,
                        if count == 1 { "" } else { "s" }
                    ));
                }

                output
            }
        },
    }
}
