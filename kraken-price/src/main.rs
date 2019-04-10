use std::env;
use requests;
use requests::ToJson;

fn main() {
    let args: Vec<String> = env::args().collect();

    let coin = &args[1];
    println!("the coin requested is {}", coin);

    let url = "https://api.kraken.com/0/public/Ticker?pair=XBTUSD";
    let response = requests::get(url).unwrap();


    // println!("{}", response.status_code());


    let data = response.json().unwrap();
    println!("$ {}", data["result"]["XXBTZUSD"]["c"][0]);
}
