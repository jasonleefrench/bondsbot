use thousands::Separable;
use crate::models::{Bond, Winner};

pub fn check_winners(my_bonds: &[Bond], winners: &[Winner], verbose: bool) {
    let mut has_won = false;
    let mut total_won = 0u64;
    if verbose {
        println!("Checking {} bonds against {} winners...", my_bonds.len(), winners.len());
    }
    let matches: Vec<_> = winners.iter()
        .filter_map(|winner| {
            my_bonds.iter().find_map(|bond| {
                if verbose {
                    println!("Checking bond group {} against winner {}...", bond.prefix, winner.winning_bond);
                }
                let prefix_len = bond.prefix.len();
                
                // Check bounds before slicing
                if winner.winning_bond.len() < prefix_len {
                    return None;
                }
                
                let winner_prefix = &winner.winning_bond[..prefix_len];
                if winner_prefix != bond.prefix {
                    return None;
                }
                
                if winner.winning_bond.len() <= prefix_len {
                    return None;
                }
                
                let winning_number = winner.winning_bond[prefix_len..].parse::<u64>().ok()?;
                
                if winning_number >= bond.start && winning_number <= bond.end {
                    Some(winner)
                } else {
                    None
                }
            })
        })
        .collect();

    for winner in &matches {
        println!("You have won {} with bond {}", winner.prize_value_str, winner.winning_bond);
        total_won += winner.prize_value;
        has_won = true;
    }
    if !has_won {
        println!("No winning bonds found.");
    } else {
        println!("Total winnings: £{}", total_won.separate_with_commas());
    }
}

