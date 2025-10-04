use dotenvy::dotenv;
use rust_kindle_sender::{run, Configuration};

fn main() {
    dotenv().ok();
    let configuration = Configuration::new();
    run(configuration);
}