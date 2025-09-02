use crate::models::Winner;

pub fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let html_content = response.text()?;
    Ok(html_content)
}

pub fn get_winners(html: &str) -> Result<Vec<Winner>, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(html);
    let winner_selector = scraper::Selector::parse("tr")
        .map_err(|e| format!("Failed to parse selector: {}", e))?;
    let mut res = Vec::new();

    let td_selector = scraper::Selector::parse("td")
        .map_err(|e| format!("Failed to parse td selector: {}", e))?;

    for html_row in document.select(&winner_selector) {
        let cells: Vec<String> = html_row
            .select(&td_selector)
            .map(|td| {
                td.value()
                    .attr("data-sort")
                    .map(|v| v.trim().to_string())
                    .unwrap_or_else(|| {
                        let text: String = td.text().collect();
                        text.trim().to_string()
                    })
            })
            .collect();

        if let Some(bond) = cells.get(1) {
             if !bond.is_empty() {
                let prize_value_raw = cells.get(0).cloned().unwrap_or_default();
                let prize_value = prize_value_raw.trim();
                res.push(Winner {
                    prize_value_str: format!("£{}", prize_value),
                    prize_value: prize_value.replace(',', "").parse::<u64>().unwrap_or(0),
                    winning_bond: bond.to_string(),
                });
            }
        }
    }

    Ok(res)
}

pub fn get_month(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(html);
    let month_selector = scraper::Selector::parse("h1.high-value-header")
        .map_err(|e| format!("Failed to parse selector: {}", e))?;

    if let Some(element) = document.select(&month_selector).next() {
        let text: String = element.text().collect();
        let month = text.trim().to_string().replace("'s high value winners", "");
        return Ok(month);
    }

    Err("Month not found in HTML".into())
}
