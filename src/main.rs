use crate::cli::get_arguments;
use crab_courier::Secrets;
use crab_courier::run;
use dotenvy::dotenv;
use std::process;

mod cli;

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
    let env_variables = Secrets::default();
    let args = get_arguments();
    println!("{CRAB_COURIER_LOGO}\nPreparing the email...");

    match run(env_variables, &args.path_to_ebook) {
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
