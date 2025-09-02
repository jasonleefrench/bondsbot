use clap::Parser;
use bondsbot::{parse_bonds, get_html, get_winners, get_month, check_winners};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    bonds: String,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let bonds = parse_bonds(&args.bonds)?;
        
    for bond in &bonds {
        bond.validate()
            .map_err(|e| format!("Invalid bond data: {e}"))?;
    }

    let html = get_html("https://www.nsandi.com/prize-checker/winners")?;

    if args.verbose {
        let month = get_month(&html)?;
        println!("Checking winners for: {month}");
    }

    let winners = get_winners(&html)?;  

    check_winners(&bonds, &winners, &args.verbose);
    Ok(())
}