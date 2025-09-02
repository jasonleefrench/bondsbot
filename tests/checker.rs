use bondsbot::models::{Bond, Winner};
use bondsbot::checker::check_winners;

#[test]
fn test_check_winners_matching_bond() {
    let bonds = vec![Bond {
        prefix: "123AB".to_string(),
        start: 456000,
        end: 456999,
    }];
    let winners = vec![Winner {
        prize_value_str: "£1000".to_string(),
        prize_value: 1000,
        winning_bond: "123AB456789".to_string(),
    }];
    
    check_winners(&bonds, &winners, &true);
}

#[test]
fn test_check_winners_no_match() {
    let bonds = vec![Bond {
        prefix: "999XY".to_string(),
        start: 100000,
        end: 199999,
    }];
    let winners = vec![Winner {
        prize_value_str: "£1000".to_string(),
        prize_value: 1000,
        winning_bond: "123AB456789".to_string(),
    }];
    
    check_winners(&bonds, &winners, &true);
}