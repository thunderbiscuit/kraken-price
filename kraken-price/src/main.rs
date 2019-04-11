use std::env;
use requests;
use requests::ToJson;
use webbrowser;

fn main() {
    // collect command line argument if given
    let args: Vec<String> = env::args().collect();

    // if no argument was given
    if args.len() < 2 {
        let price: f64 = query_api(&String::from("btc")).parse().unwrap();
        println!("$ {}", price);
    }

    // if argument was given
    if args.len() > 1 {
        let arg = &args[1];

        // execute query on bitcoin and doge, Rick Roll anything else
        match arg.as_str() {
            "--help" => {
                println!("krakenprice");
                println!("Query the Kraken public API to get the latest bitcoin or doge price.");
                println!("The default coin (no arguments to the command) is bitcoin.");
                println!();
                println!("USAGE:");
                println!("    krakenprice [coin] [--] [args]");
                println!();
                println!("OPTIONS:");
                println!("    --help          get to this help documentation");
            }
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
}



#[allow(non_snake_case)]
fn query_api(coin: &String) -> String {
    let baseURL = "https://api.kraken.com/0/public/Ticker?pair=";
    let btcusd   = "XXBTZUSD";
    let dogebtc  = "XXDGXXBT";
    let price: String;

    match coin.as_str() {
        "btc" | "bitcoin" => {
            let url = [baseURL, btcusd].concat();
            let response = requests::get(url).unwrap();
            let full_data = response.json().unwrap();
            price = full_data["result"]["XXBTZUSD"]["c"][0].to_string();
        }
        "doge" => {
            let url = [baseURL, dogebtc].concat();
            let response = requests::get(url).unwrap();
            let full_data = response.json().unwrap();
            price = full_data["result"]["XXDGXXBT"]["c"][0].to_string();
        }
        _ => {
            price = String::from("not a valid coin");
        }
    }

    price
}