pub(crate) enum Command {
    Set(String, String),
    Get(String),
    Del(String),
    Ping,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut input_iter = input.split_ascii_whitespace();

        match input_iter.next() {
            Some("GET") => {
                let key = input_iter.next();

                if let Some(key) = key {
                    if input_iter.next().is_some() {
                        Err("Multiple keys not allowed".to_string())
                    } else {
                        Ok(Self::Get(key.to_string()))
                    }
                } else {
                    Err("Key is empty".to_string())
                }
            }
            Some("SET") => {
                let key = input_iter.next();

                if let Some(key) = key {
                    let value = input_iter.next();

                    if let Some(value) = value {
                        if input_iter.next().is_some() {
                            Err("Multiple values not allowed".to_string())
                        } else {
                            Ok(Self::Set(key.to_string(), value.to_string()))
                        }
                    } else {
                        Err("Value is empty".to_string())
                    }
                } else {
                    Err("Key is empty".to_string())
                }
            }
            Some("DEL") => {
                let key = input_iter.next();

                if let Some(key) = key {
                    if input_iter.next().is_some() {
                        Err("Multiple keys not allowed".to_string())
                    } else {
                        Ok(Self::Del(key.to_string()))
                    }
                } else {
                    Err("Key is empty".to_string())
                }
            }
            Some("PING") => {
                if input_iter.next().is_some() {
                    Err("Invalid ping format".to_string())
                } else {
                    Ok(Self::Ping)
                }
            }
            _ => Err("Invalid Request".to_string()),
        }
    }
}
