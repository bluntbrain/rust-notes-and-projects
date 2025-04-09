use core::{error::Error, result::Result::Ok};
use std::env;

use chrono::prelude::*;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let local: DateTime<Local> = Local::now();
    let utc = Utc::now();
    let url = env::var("FINAL_URL");
    match url {
        Ok(res) => println!("{}", res),
        Err(e) => println!("Error {}", e),
    }
    print!("{} {}", local, utc);
}
