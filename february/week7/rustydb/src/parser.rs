/// A condition used in WHERE clauses: `column = value`
pub struct Condition {
    pub column: String,
    pub value: String,
}

/// The parsed SQL statement variants we support.
pub enum Statement {
    Select {
        table: String,
        columns: Vec<String>, // ["*"] means all columns
        condition: Option<Condition>,
    },
    Insert {
        table: String,
        #[allow(dead_code)]
        columns: Vec<String>,
        values: Vec<String>,
    },
}

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

/// Split the raw input into tokens, handling:
///  - Quoted strings (single-quoted) as one token
///  - Parentheses / commas as individual tokens
///  - Stripping trailing semicolons
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Single-quoted string → one token (without the quotes)
            '\'' => {
                chars.next(); // consume opening quote
                let mut s = String::new();
                for c in chars.by_ref() {
                    if c == '\'' {
                        break;
                    }
                    s.push(c);
                }
                tokens.push(s);
            }
            // Punctuation that is its own token
            '(' | ')' | ',' | ';' => {
                chars.next();
                // Silently drop semicolons — they are just statement terminators
                if ch != ';' {
                    tokens.push(ch.to_string());
                }
            }
            // Regular word token
            _ => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ' '
                        || c == '\t'
                        || c == '\n'
                        || c == '\r'
                        || c == '('
                        || c == ')'
                        || c == ','
                        || c == ';'
                    {
                        break;
                    }
                    s.push(c);
                    chars.next();
                }
                tokens.push(s);
            }
        }
    }
    tokens
}

// ---------------------------------------------------------------------------
// Parser helpers
// ---------------------------------------------------------------------------

struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<String>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(|s| s.as_str())
    }

    fn next_token(&mut self) -> Option<&str> {
        let tok = self.tokens.get(self.pos).map(|s| s.as_str());
        self.pos += 1;
        tok
    }

    /// Consume the next token and assert (case-insensitive) it equals `expected`.
    fn expect_keyword(&mut self, expected: &str) -> Result<(), String> {
        match self.next_token() {
            Some(tok) if tok.to_lowercase() == expected.to_lowercase() => Ok(()),
            Some(tok) => Err(format!(
                "Syntax error: expected '{}', found '{}'",
                expected, tok
            )),
            None => Err(format!(
                "Syntax error: expected '{}', found end of input",
                expected
            )),
        }
    }

    /// Consume the next token and return it (error if missing).
    fn expect_any(&mut self, role: &str) -> Result<String, String> {
        match self.next_token() {
            Some(tok) => Ok(tok.to_string()),
            None => Err(format!(
                "Syntax error: expected {}, found end of input",
                role
            )),
        }
    }

    // -----------------------------------------------------------------------
    // Parse a comma-separated list of identifiers enclosed in parentheses
    // e.g.  ( col1 , col2 , col3 )
    // -----------------------------------------------------------------------
    fn parse_paren_list(&mut self) -> Result<Vec<String>, String> {
        self.expect_keyword("(")?;
        let mut items = Vec::new();
        loop {
            match self.peek() {
                Some(")") => {
                    self.next_token();
                    break;
                }
                Some(",") => {
                    self.next_token();
                }
                Some(_) => {
                    let item = self.expect_any("identifier")?;
                    items.push(item);
                }
                None => return Err("Syntax error: unclosed parenthesis".to_string()),
            }
        }
        Ok(items)
    }

    // -----------------------------------------------------------------------
    // Parse a comma-separated list of column names (no parens)
    // Stops at a keyword that is NOT a comma, e.g. FROM / WHERE / end
    // -----------------------------------------------------------------------
    fn parse_column_list(&mut self) -> Result<Vec<String>, String> {
        let mut cols = Vec::new();
        loop {
            let col = self.expect_any("column name")?;
            cols.push(col);
            match self.peek() {
                Some(",") => {
                    self.next_token(); // consume comma, continue
                }
                _ => break,
            }
        }
        Ok(cols)
    }

    // -----------------------------------------------------------------------
    // Parse optional WHERE clause → Condition
    // Syntax: WHERE column = value
    // -----------------------------------------------------------------------
    fn parse_where(&mut self) -> Result<Option<Condition>, String> {
        match self.peek() {
            Some(kw) if kw.to_lowercase() == "where" => {
                self.next_token(); // consume WHERE
                let column = self.expect_any("column name")?;
                self.expect_keyword("=")?;
                let value = self.expect_any("value")?;
                Ok(Some(Condition { column, value }))
            }
            _ => Ok(None),
        }
    }

    // -----------------------------------------------------------------------
    // SELECT col1, col2 FROM table [WHERE col = val]
    // SELECT * FROM table [WHERE col = val]
    // -----------------------------------------------------------------------
    fn parse_select(&mut self) -> Result<Statement, String> {
        // Column list — could be * or comma-separated names
        let columns = self.parse_column_list()?;

        self.expect_keyword("from")?;
        let table = self.expect_any("table name")?;

        let condition = self.parse_where()?;

        Ok(Statement::Select {
            table,
            columns,
            condition,
        })
    }

    // -----------------------------------------------------------------------
    // INSERT INTO table (col1, col2) VALUES (val1, val2)
    // -----------------------------------------------------------------------
    fn parse_insert(&mut self) -> Result<Statement, String> {
        self.expect_keyword("into")?;
        let table = self.expect_any("table name")?;

        // Optional explicit column list
        let columns = if self.peek() == Some("(") {
            self.parse_paren_list()?
        } else {
            vec!["*".to_string()]
        };

        self.expect_keyword("values")?;
        let values = self.parse_paren_list()?;

        if columns[0] != "*" && columns.len() != values.len() {
            return Err(format!(
                "Column count ({}) does not match value count ({})",
                columns.len(),
                values.len()
            ));
        }

        Ok(Statement::Insert {
            table,
            columns,
            values,
        })
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

impl Statement {
    /// Parse a SQL string into a `Statement`.
    ///
    /// Supported syntax:
    /// ```
    /// INSERT INTO table_name (col1, col2) VALUES (val1, val2);
    /// SELECT * FROM table_name;
    /// SELECT col1, col2 FROM table_name WHERE col = val;
    /// ```
    pub fn parse(input: &str) -> Result<Self, String> {
        let tokens = tokenize(input);
        if tokens.is_empty() {
            return Err("Empty statement".to_string());
        }

        let mut parser = Parser::new(tokens);
        let keyword = parser.expect_any("statement keyword")?;

        match keyword.to_lowercase().as_str() {
            "select" => parser.parse_select(),
            "insert" => parser.parse_insert(),
            other => Err(format!("Unknown statement: '{}'", other)),
        }
    }
}
