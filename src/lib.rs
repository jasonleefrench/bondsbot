pub mod models;
pub mod parser;
pub mod scraper;
pub mod checker;

pub use models::{Bond, Winner};
pub use parser::parse_bonds;
pub use scraper::{get_html, get_winners, get_month};
pub use checker::check_winners;