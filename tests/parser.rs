use bondsbot::parser::{parse_bonds, parse_bond_number, ranges_overlap};

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