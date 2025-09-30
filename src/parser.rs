use crate::models::Bond;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn parse_bonds_from_file(file_path: &str) -> Result<Vec<Bond>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;
    parse_bonds(&content)
}

pub fn parse_bonds(bonds_str: &str) -> Result<Vec<Bond>, Box<dyn std::error::Error>> {
    // Check if this looks like a file path
    if Path::new(bonds_str).is_file() {
        return parse_bonds_from_file(bonds_str);
    }

    let mut bonds = Vec::new();

    for bond_item in bonds_str.split(',') {
        let bond_item = bond_item.trim();

        match bond_item.split('-').collect::<Vec<&str>>().as_slice() {
            [single_bond] => {
                // Single bond
                let (prefix, number) = parse_bond_number(single_bond)?;
                let bond = Bond {
                    prefix,
                    start: number,
                    end: number,
                };
                bond.validate()
                    .map_err(|e| format!("Bond validation failed: {e}"))?;
                bonds.push(bond);
            }
            [start_bond, end_bond] => {
                // Bond range

                let (prefix, start_num) = parse_bond_number(start_bond)?;
                let (end_prefix, end_num) = parse_bond_number(end_bond)?;

                if prefix != end_prefix {
                    return Err(
                        format!("Prefix mismatch in range: {prefix} vs {end_prefix}").into(),
                    );
                }

                let bond = Bond {
                    prefix,
                    start: start_num,
                    end: end_num,
                };

                bond.validate()
                    .map_err(|e| format!("Bond validation failed: {e}"))?;
                bonds.push(bond);
            }
            _ => return Err(format!("Invalid bond format: {bond_item}").into()),
        }
    }

    check_for_duplicates(&bonds)?;

    Ok(bonds)
}

fn check_for_duplicates(bonds: &[Bond]) -> Result<(), Box<dyn std::error::Error>> {
    let mut seen_bonds = HashSet::new();
    let mut duplicate_bonds = Vec::new();

    for bond in bonds {
        for num in bond.start..=bond.end {
            let bond_id = format!("{}{}", bond.prefix, num);
            if !seen_bonds.insert(bond_id.clone()) {
                duplicate_bonds.push(bond_id);
            }
        }
    }

    if !duplicate_bonds.is_empty() {
        let first_few = duplicate_bonds
            .iter()
            .take(5)
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let msg = match duplicate_bonds.len() {
            n if n > 5 => format!(
                "Duplicate bonds detected: {} (and {} more)",
                first_few,
                n - 5
            ),
            _ => format!("Duplicate bonds detected: {first_few}"),
        };
        return Err(msg.into());
    }

    for (i, bond) in bonds.iter().enumerate() {
        for other in bonds.iter().skip(i + 1) {
            if bond.prefix == other.prefix
                && ranges_overlap(bond.start, bond.end, other.start, other.end)
            {
                let overlap_start = bond.start.max(other.start);
                let overlap_end = bond.end.min(other.end);
                return Err(format!(
                        "Overlapping bond ranges detected: {}{}-{}{} and {}{}-{}{} (overlap: {}{}-{}{})",
                        bond.prefix, bond.start, bond.prefix, bond.end,
                        other.prefix, other.start, other.prefix, other.end,
                        bond.prefix, overlap_start, bond.prefix, overlap_end
                    ).into());
            }
        }
    }

    Ok(())
}

pub fn ranges_overlap(start1: u64, end1: u64, start2: u64, end2: u64) -> bool {
    !(end1 < start2 || end2 < start1)
}

pub fn parse_bond_number(bond_str: &str) -> Result<(String, u64), Box<dyn std::error::Error>> {
    // Find the last letter position to split prefix from number
    let last_letter_index = bond_str
        .char_indices()
        .filter(|(_, ch)| ch.is_ascii_alphabetic())
        .next_back()
        .map(|(i, _)| i)
        .ok_or_else(|| format!("No letters found in bond: {bond_str}"))?;

    let split_index = last_letter_index + 1;

    if split_index >= bond_str.len() {
        return Err(format!("No number part found in bond: {bond_str}").into());
    }

    let prefix = bond_str[..split_index].to_string();
    let number_part = &bond_str[split_index..];

    if number_part.is_empty() {
        return Err(format!("No number part found in bond: {bond_str}").into());
    }

    let number = number_part
        .parse::<u64>()
        .map_err(|_| format!("Invalid number in bond: {bond_str}"))?;

    Ok((prefix, number))
}
