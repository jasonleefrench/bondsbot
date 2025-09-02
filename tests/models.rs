use bondsbot::models::Bond;

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