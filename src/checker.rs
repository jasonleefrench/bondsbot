use thousands::Separable;
use crate::models::{Bond, Winner};

pub fn check_winners(my_bonds: &[Bond], winners: &[Winner], verbose: &bool) {
    let mut has_won = false;
    let mut total_won = 0u64;
    if *verbose {
        println!("Checking {} bonds against {} winners...", my_bonds.len(), winners.len());
    }
    for winner in winners {
        for bond in my_bonds {
            if *verbose {
                print!("Checking bond group {} against winner {}...\n", bond.prefix, winner.winning_bond);
            }
            let prefix_len = bond.prefix.len();
            
            // Check bounds before slicing
            if winner.winning_bond.len() < prefix_len {
                continue;
            }
            
            let winner_prefix = &winner.winning_bond[..prefix_len];
            if winner_prefix == bond.prefix {
                if winner.winning_bond.len() > prefix_len {
                    if let Ok(winning_number) = winner.winning_bond[prefix_len..].parse::<u64>() {
                        if winning_number >= bond.start && winning_number <= bond.end {
                            println!("You have won {} with bond {}", winner.prize_value_str, winner.winning_bond);
                            has_won = true;
                            total_won += winner.prize_value;
                        }
                    }
                }
            }
        }
    }
    if !has_won {
        println!("No winning bonds found.");
    } else {
        println!("Total winnings: £{}", total_won.separate_with_commas());
    }
}

