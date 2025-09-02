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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_validation_valid() {
        let bond = Bond {
            prefix: "123AB".to_string(),
            start: 1000,
            end: 9999,
        };
        assert!(bond.validate().is_ok());
    }

    #[test]
    fn test_bond_validation_empty_prefix() {
        let bond = Bond {
            prefix: "".to_string(),
            start: 1000,
            end: 9999,
        };
        assert!(bond.validate().is_err());
    }

    #[test]
    fn test_bond_validation_start_greater_than_end() {
        let bond = Bond {
            prefix: "123AB".to_string(),
            start: 9999,
            end: 1000,
        };
        assert!(bond.validate().is_err());
    }
}