use clap::{ArgGroup, Parser};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("action").required(true).multiple(false)))]
pub struct Args {
    #[arg(long, short = 'e', required = true, help = "Path to an executable")]
    pub exec: Option<String>,

    #[arg(long, short = 'w', group = "action", help = "Path to a wordlist")]
    pub wordlist: Option<String>,

    #[arg(
        long,
        short = 's',
        group = "action",
        help = "Throws a list of strings with a length of the specified range"
    )]
    pub string_range: Option<String>,

    #[arg(
        long,
        short = 'n',
        group = "action",
        help = "Throws a list of numbers from the specified range"
    )]
    pub number_range: Option<String>,

    #[arg(
        long,
        short = 'i',
        help = "Defines by how much 'n' or 's' will increment",
        default_value_t = 1
    )]
    pub increment: usize,

    #[arg(
        long,
        short = 'a',
        help = "List of arguments to be passed to the executable. \"FUZZ\" string will be replaced by one of the options (-w/-s/-n) described above"
    )]
    pub arguments: Option<String>,
}

impl Args {
    pub fn new() -> Result<Args, Box<dyn Error>> {
        Ok(Args::parse())
    }
}
