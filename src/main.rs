use crate::cli::get_arguments;
use crab_courier::EnvVariables;
use crab_courier::run;
use dotenvy::dotenv;

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
    // TODO tests
    dotenv().ok();
    let env_variables = EnvVariables::default();
    let args = get_arguments();
    println!("{CRAB_COURIER_LOGO}");
    run(env_variables, &args.path_to_ebook);
}
