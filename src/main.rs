use dotenvy::dotenv;
use crab_courier::{Configuration, run};

fn main() {
    // TODO tests
    dotenv().ok();
    let configuration = Configuration::new();
    run(configuration);
}
