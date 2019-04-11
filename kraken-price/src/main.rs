use std::env;
use requests;
use requests::ToJson;
use webbrowser;

fn main() {
    // collect command line argument if given
    let args: Vec<String> = env::args().collect();

    // if argument was given put it into variable arg
    let arg = &args[1];

    // execute query on bitcoin and doge, Rick Roll anything else
    match arg.as_str() {
        "btc" | "bitcoin" => {
            let price: f64 = query_api(arg).parse().unwrap();
            println!("$ {}", price);
        }
        "doge" => {
            let price: f64 = query_api(arg).parse().unwrap();
            let price_sats: u32 = (price * 100000000.0) as u32;
            println!("{} sats", price_sats);
        }
        _ => {
            webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_ok();
            println!("Surprise monkeyfighter.");
        }
    }
}




fn query_api(coin: &String) -> String {
    // let baseURL = "https://api.kraken.com/0/public/Ticker?pair=";

    // let response;
    let price;

    match coin.as_str() {
        "btc" | "bitcoin" => {
            let url = concat!("https://api.kraken.com/0/public/Ticker?pair=", "XBTUSD");
            let response = requests::get(url).unwrap();
            let full_data = response.json().unwrap();
            price = full_data["result"]["XXBTZUSD"]["c"][0].to_string();
        }
        "doge" => {
            let url = concat!("https://api.kraken.com/0/public/Ticker?pair=", "XXDGXXBT");
            let response = requests::get(url).unwrap();
            let full_data = response.json().unwrap();
            price = full_data["result"]["XXDGXXBT"]["c"][0].to_string();
        }
        _ => {
            price = String::from("not a valid coin");
        }
    }



    // println!("{}", response.status_code());
    price
}