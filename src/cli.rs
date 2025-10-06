use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Arguments {
    pub(crate) path_to_ebook: String
}


pub(crate) fn get_arguments() -> Arguments {
    Arguments::parse()
}

