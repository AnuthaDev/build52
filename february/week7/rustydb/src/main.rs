use storage::Database;

mod parser;
mod server;
mod storage;

const STUDENT_TABLE: &str = "Students";

fn main() {
    let mut db = Database::new();
    db.create_table(
        STUDENT_TABLE,
        vec!["id".to_string(), "name".to_string(), "class".to_string()],
    );

    let srv = server::Server::bind("127.0.0.1:7878", db).expect("Failed to bind TCP listener");
    srv.run();
}
