use crate::models::Bond;
use std::collections::HashSet;

pub fn parse_bonds(bonds_str: &str) -> Result<Vec<Bond>, Box<dyn std::error::Error>> {
    let mut bonds = Vec::new();
    
    for bond_item in bonds_str.split(',') {
        let bond_item = bond_item.trim();
        
        let parts: Vec<&str> = bond_item.split('-').collect();
        
        if parts.len() == 1 {
            // Single bond
            let (prefix, number) = parse_bond_number(parts[0])?;
            let bond = Bond {
                prefix,
                start: number,
                end: number,
            };
            bond.validate().map_err(|e| format!("Bond validation failed: {}", e))?;
            bonds.push(bond);
        } else if parts.len() == 2 {
            // Bond range
            let start_bond = parts[0];
            let end_bond = parts[1];
            
            let (prefix, start_num) = parse_bond_number(start_bond)?;
            let (end_prefix, end_num) = parse_bond_number(end_bond)?;
            
            if prefix != end_prefix {
                return Err(format!("Prefix mismatch in range: {} vs {}", prefix, end_prefix).into());
            }
            
            let bond = Bond {
                prefix,
                start: start_num,
                end: end_num,
            };
            
            bond.validate().map_err(|e| format!("Bond validation failed: {}", e))?;
            bonds.push(bond);
        } else {
            return Err(format!("Invalid bond format: {}", bond_item).into());
        }
    }
    
    check_for_duplicates(&bonds)?;
    
    Ok(bonds)
}

fn check_for_duplicates(bonds: &[Bond]) -> Result<(), Box<dyn std::error::Error>> {
    let mut seen_bonds = HashSet::new();
    let mut duplicate_bonds = Vec::new();
    
    for bond in bonds.iter() {
        for num in bond.start..=bond.end {
            let bond_id = format!("{}{}", bond.prefix, num);
            if !seen_bonds.insert(bond_id.clone()) {
                duplicate_bonds.push(bond_id);
            }
        }
    }
    
    if !duplicate_bonds.is_empty() {
        let first_few = duplicate_bonds.iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        
        let msg = if duplicate_bonds.len() > 5 {
            format!("Duplicate bonds detected: {} (and {} more)", first_few, duplicate_bonds.len() - 5)
        } else {
            format!("Duplicate bonds detected: {}", first_few)
        };
        return Err(msg.into());
    }
    
    for i in 0..bonds.len() {
        for j in (i + 1)..bonds.len() {
            let bond = &bonds[i];
            let other = &bonds[j];
            if bond.prefix == other.prefix {
                if ranges_overlap(bond.start, bond.end, other.start, other.end) {
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
    }
    
    Ok(())
}

fn ranges_overlap(start1: u64, end1: u64, start2: u64, end2: u64) -> bool {
    !(end1 < start2 || end2 < start1)
}

fn parse_bond_number(bond_str: &str) -> Result<(String, u64), Box<dyn std::error::Error>> {
    // Find the last letter position to split prefix from number
    let mut last_letter_index = None;
    
    for (i, ch) in bond_str.chars().enumerate() {
        if ch.is_ascii_alphabetic() {
            last_letter_index = Some(i);
        }
    }
    
    let last_letter_index = last_letter_index
        .ok_or_else(|| format!("No letters found in bond: {}", bond_str))?;
    
    let split_index = last_letter_index + 1;
    
    if split_index >= bond_str.len() {
        return Err(format!("No number part found in bond: {}", bond_str).into());
    }
    
    let prefix = bond_str[..split_index].to_string();
    let number_part = &bond_str[split_index..];
    
    if number_part.is_empty() {
        return Err(format!("No number part found in bond: {}", bond_str).into());
    }
    
    let number = number_part.parse::<u64>()
        .map_err(|_| format!("Invalid number in bond: {}", bond_str))?;
    
    Ok((prefix, number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bond_number() {
        let result = parse_bond_number("632QA322573");
        if result.is_err() {
            println!("Error: {}", result.as_ref().unwrap_err());
        }
        assert!(result.is_ok());
        let (prefix, number) = result.unwrap();
        assert_eq!(prefix, "632QA");
        assert_eq!(number, 322573);
    }

    #[test]
    fn test_parse_bond_number_invalid() {
        let result = parse_bond_number("ABCD");
        assert!(result.is_err());
        
        let result = parse_bond_number("123456");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bonds_single_range() {
        let result = parse_bonds("632QA322573-632QA322622");
        assert!(result.is_ok());
        let bonds = result.unwrap();
        assert_eq!(bonds.len(), 1);
        assert_eq!(bonds[0].prefix, "632QA");
        assert_eq!(bonds[0].start, 322573);
        assert_eq!(bonds[0].end, 322622);
    }

    #[test]
    fn test_parse_bonds_multiple_ranges() {
        let result = parse_bonds("632QA322573-632QA322622,632PX825462-632PX825486");
        assert!(result.is_ok());
        let bonds = result.unwrap();
        assert_eq!(bonds.len(), 2);
        
        assert_eq!(bonds[0].prefix, "632QA");
        assert_eq!(bonds[0].start, 322573);
        assert_eq!(bonds[0].end, 322622);
        
        assert_eq!(bonds[1].prefix, "632PX");
        assert_eq!(bonds[1].start, 825462);
        assert_eq!(bonds[1].end, 825486);
    }

    #[test]
    fn test_parse_bonds_prefix_mismatch() {
        let result = parse_bonds("632QA322573-632PX322622");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bonds_single_bond() {
        let result = parse_bonds("224BZ748917");
        assert!(result.is_ok());
        let bonds = result.unwrap();
        assert_eq!(bonds.len(), 1);
        assert_eq!(bonds[0].prefix, "224BZ");
        assert_eq!(bonds[0].start, 748917);
        assert_eq!(bonds[0].end, 748917);
    }

    #[test]
    fn test_parse_bonds_mixed_single_and_range() {
        let result = parse_bonds("224BZ748917,632QA322573-632QA322622,420AB123456");
        assert!(result.is_ok());
        let bonds = result.unwrap();
        assert_eq!(bonds.len(), 3);
        
        // Single bond
        assert_eq!(bonds[0].prefix, "224BZ");
        assert_eq!(bonds[0].start, 748917);
        assert_eq!(bonds[0].end, 748917);
        
        // Range
        assert_eq!(bonds[1].prefix, "632QA");
        assert_eq!(bonds[1].start, 322573);
        assert_eq!(bonds[1].end, 322622);
        
        // Single bond
        assert_eq!(bonds[2].prefix, "420AB");
        assert_eq!(bonds[2].start, 123456);
        assert_eq!(bonds[2].end, 123456);
    }

    #[test]
    fn test_parse_bonds_invalid_format() {
        let result = parse_bonds("632QA322573-632QA322622-extra");
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_single_bonds() {
        let result = parse_bonds("224BZ748917,224BZ748917");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Duplicate bonds detected"));
        assert!(err_msg.contains("224BZ748917"));
    }

    #[test]
    fn test_duplicate_in_range() {
        let result = parse_bonds("632QA322570-632QA322575,632QA322573");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Duplicate bonds detected"));
        assert!(err_msg.contains("632QA322573"));
    }

    #[test]
    fn test_overlapping_ranges() {
        let result = parse_bonds("632QA322570-632QA322580,632QA322575-632QA322585");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Duplicate bonds detected") || err_msg.contains("Overlapping bond ranges detected"));
    }

    #[test]
    fn test_fully_contained_range() {
        let result = parse_bonds("632QA322570-632QA322590,632QA322575-632QA322585");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Duplicate bonds detected") || err_msg.contains("Overlapping bond ranges detected"));
    }

    #[test]
    fn test_adjacent_ranges_no_overlap() {
        let result = parse_bonds("632QA322570-632QA322580,632QA322581-632QA322590");
        assert!(result.is_ok());
    }

    #[test]
    fn test_different_prefix_no_overlap() {
        let result = parse_bonds("632QA322570-632QA322580,632QB322570-632QB322580");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_duplicates() {
        let result = parse_bonds("224BZ748917,632QA322573,224BZ748917,632QA322573,420AB123456");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Duplicate bonds detected"));
    }

    #[test]
    fn test_ranges_overlap_helper() {
        assert!(ranges_overlap(1, 5, 3, 7));
        assert!(ranges_overlap(3, 7, 1, 5));
        assert!(ranges_overlap(1, 10, 5, 6));
        assert!(ranges_overlap(5, 6, 1, 10));
        assert!(!ranges_overlap(1, 5, 6, 10));
        assert!(!ranges_overlap(6, 10, 1, 5));
        assert!(ranges_overlap(1, 5, 5, 10));
    }
}