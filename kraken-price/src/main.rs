use std::env;
use requests;
use requests::ToJson;

fn main() {
    // collect command line argument if given
    let args: Vec<String> = env::args().collect();

    // if argument was given put it into variable arg
    let arg1 = &args[1];

    // execute query on bitcoin and doge, Rick Roll anything else
    match arg1.as_str() {
        "btc" => {
            let price = query_api(arg1);
            println!("{}", price);
        },
        _ => println!("something else")
    }
}




fn query_api(coin: &String) -> String {
    // let baseURL = "https://api.kraken.com/0/public/Ticker?pair=";
    // let url = "https://api.kraken.com/0/public/Ticker?pair=XBTUSD";

    // let response;
    let price;

    match coin.as_str() {
        "btc" => {
            let url = concat!("https://api.kraken.com/0/public/Ticker?pair=", "XBTUSD");
            let response = requests::get(url).unwrap();
            let full_data = response.json().unwrap();
            price = full_data["result"]["XXBTZUSD"]["c"][0].to_string();
        },
        _ => {
            price = String::from("not a valid coin");
            println!("not a coin");
        }
    }



    // println!("{}", response.status_code());
    // println!("$ {}", price);
    price
}