use crab_courier::{get_arguments, run};
use dotenvy::dotenv;
use std::process;
use std::time::Duration;
use indicatif::ProgressBar;

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
    println!("{CRAB_COURIER_LOGO}");

    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Preparing the email...");
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    match run(&args) {
        Ok(_) => {
            progress_bar.finish_with_message("Email sent successfully!");
        }
        Err(error) => {
            eprintln!(
                "Failed to send the email with attachment to the recipient. Cause: {error:?}"
            );
            process::exit(1);
        }
    }

}
