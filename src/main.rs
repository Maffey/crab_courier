use dotenvy::dotenv;
use rust_kindle_sender::{Configuration, run};

fn main() {
    dotenv().ok();
    let configuration = Configuration::new();
    run(configuration);
}
