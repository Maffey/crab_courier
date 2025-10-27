use crab_courier::{get_arguments, run};
use dotenvy::dotenv;
use std::process;

const CRAB_COURIER_LOGO: &str = r#"
_________              ___.   _________                     .__
\_   ___ \____________ \_ |__ \_   ___ \  ____  __ _________|__| ___________
/    \  \/\_  __ \__  \ | __ \/    \  \/ /  _ \|  |  \_  __ \  |/ __ \_  __ \
\     \____|  | \// __ \| \_\ \     \___(  <_> )  |  /|  | \/  \  ___/|  | \/
 \______  /|__|  (____  /___  /\______  /\____/|____/ |__|  |__|\___  >__|
        \/            \/    \/        \/                            \/
"#;

fn main() {
    dotenv().ok();
    let args = get_arguments();
    println!("{CRAB_COURIER_LOGO}\nPreparing the email...");

    match run(&args) {
        Ok(_) => {
            println!("Email has been sent successfully!");
        }
        Err(error) => {
            eprintln!(
                "Failed to send the email with attachment to the recipient. Cause: {error:?}"
            );
            process::exit(1);
        }
    }
}
