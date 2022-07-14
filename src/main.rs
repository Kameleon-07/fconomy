use clap::Parser;
use scraper::{Html, Selector};

#[derive(Debug, Parser)]
#[clap(author = "Kameleon-07")]
struct Args {
    from: String,
    to: String,

    #[clap(default_value_t = 1)]
    amount: i32,
}
fn main() {
    let args = Args::parse();

    let result = reqwest::blocking::get(
        format!("https://www.xe.com/currencyconverter/convert/?Amount={}&From={}&To={}",
        args.amount,
        args.from.to_uppercase(),
        args.to.to_uppercase()
    ))
    .unwrap()
    .text()
    .unwrap();

    let document = Html::parse_document(&result);
    
    let money_selector = Selector::parse("p.result__BigRate-sc-1bsijpp-1").unwrap();

    let converted_money = document.select(&money_selector).map(|x| x.inner_html());

    let mut money = String::new();

    for inner_html in converted_money {
        for ch in inner_html.chars() {
            if ch != '<' {
                money.push(ch);
            } else {
                break;
            }
        }
    }

    if money.is_empty() {
        eprintln!("There was an error while converting money");
    }

    println!("{}", money);

}
