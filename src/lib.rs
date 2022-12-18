mod args;
mod fuzzer;

use std::error::Error;

use args::Args;
use crossterm::style::Stylize;
use fuzzer::FuzzerConfig;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = match Args::new() {
        Ok(args) => args,
        Err(e) => {
            return Err(e);
        }
    };

    let fuzzer_config = args_to_fuzzer_config(&args)?;

    print_figlet();

    fuzzer::fuzz(&fuzzer_config)
}

fn args_to_fuzzer_config(args: &Args) -> Result<FuzzerConfig, Box<dyn Error>> {
    let mut fuzzer_config = FuzzerConfig::new();

    fuzzer_config.exec = match args.exec.clone() {
        Some(value) => Some(value),
        None => None,
    };

    fuzzer_config.wordlist = match args.wordlist.clone() {
        Some(value) => Some(value),
        None => None,
    };

    fuzzer_config.string_range = match args.string_range.clone() {
        Some(value) => Some(string_to_usize_range(value.as_str())?),
        None => None,
    };

    fuzzer_config.number_range = match args.number_range.clone() {
        Some(value) => Some(string_to_range(value.as_str())?),
        None => None,
    };

    if args.increment > 0 {
        fuzzer_config.increment = args.increment;
    } else {
        fuzzer_config.increment = 1;
    }

    fuzzer_config.arguments = match args.arguments.clone() {
        Some(value) => Some(value),
        None => None,
    };

    Ok(fuzzer_config)
}

fn string_to_usize_range(s: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let mut range = (0, 0);

    let range_vec: Vec<&str> = s.split("..").collect();
    if range_vec.len() == 2 {
        range.0 = range_vec[0].parse::<usize>()?;
        range.1 = range_vec[1].parse::<usize>()?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Problems with processing command-line arguments. Unable to parse the following value: \"{}\": not a range.",
                s
            ),
        )));
    }

    Ok(range)
}

fn string_to_range(s: &str) -> Result<(i128, i128), Box<dyn Error>> {
    let mut range = (0, 0);

    let range_vec: Vec<&str> = s.split("..").collect();
    if range_vec.len() == 2 {
        range.0 = range_vec[0].parse::<i128>()?;
        range.1 = range_vec[1].parse::<i128>()?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Problems with processing command-line arguments. Unable to parse the following value: \"{}\": not a range.",
                s
            ),
        )));
    }

    Ok(range)
}

fn print_figlet() {
    let figlet_1 = r#"
                              ____ _     ___ _____
                             / ___| |   |_ _|  ___|
                            | |   | |    | || |_
                            | |___| |___ | ||  _|
                             \____|_____|___|_|
"#;

    let figlet_2 = r#"
                                                    _       _ _
       ___ ___  _ __ ___  _ __ ___   __ _ _ __   __| |     | (_)_ __   ___
      / __/ _ \| '_ ` _ \| '_ ` _ \ / _` | '_ \ / _` |_____| | | '_ \ / _ \
     | (_| (_) | | | | | | | | | | | (_| | | | | (_| |_____| | | | | |  __/
      \___\___/|_| |_| |_|_| |_| |_|\__,_|_| |_|\__,_|     |_|_|_| |_|\___|"#;

    let figlet_3 = r#"
                            __
                           / _|_   _ ___________ _ __
                          | |_| | | |_  /_  / _ \ '__|
                          |  _| |_| |/ / / /  __/ |
                          |_|  \__,_/___/___\___|_|


"#;

    print!("{}", figlet_1.red());
    print!("{}", figlet_2.green());
    print!("{}", figlet_3.blue());
}
