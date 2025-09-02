use bondsbot::scraper::{get_winners, get_month};

#[test]
fn test_get_winners_empty_html() {
    let html = "<html><body></body></html>";
    let result = get_winners(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_get_winners_with_table() {
    let html = r#"
        <html><body>
            <table>
                <tr>
                    <td data-sort="1000">£1,000</td>
                    <td>123AB456789</td>
                </tr>
            </table>
        </body></html>
    "#;
    let result = get_winners(html);
    assert!(result.is_ok());
    let winners = result.unwrap();
    assert_eq!(winners.len(), 1);
    assert_eq!(winners[0].prize_value_str, "£1000");
    assert_eq!(winners[0].prize_value, 1000);
    assert_eq!(winners[0].winning_bond, "123AB456789");
}

#[test]
fn test_get_month() {
    let html = r#"
        <html><body>
            <h1 class="high-value-header">January's high value winners</h1>
        </body></html>
    "#;
    let result = get_month(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "January");
}