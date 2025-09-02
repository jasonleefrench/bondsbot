#[derive(Debug)]
pub struct Winner {
    pub prize_value_str: String,
    pub winning_bond: String,
    pub prize_value: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct Bond {
    pub prefix: String,
    pub start: u64,
    pub end: u64,
}

impl Bond {
    pub fn validate(&self) -> Result<(), String> {
        if self.prefix.is_empty() {
            return Err("Bond prefix cannot be empty".to_string());
        }
        if self.start > self.end {
            return Err(format!("Bond start ({}) cannot be greater than end ({})", self.start, self.end));
        }
        Ok(())
    }
}

