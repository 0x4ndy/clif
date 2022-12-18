use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process::Command;

use shellwords;

#[derive(Default, Debug)]
pub struct FuzzerConfig {
    pub exec: Option<String>,
    pub wordlist: Option<String>,
    pub number_range: Option<(i128, i128)>,
    pub string_range: Option<(usize, usize)>,
    pub increment: usize,
    pub arguments: Option<String>,
}

impl FuzzerConfig {
    pub fn new() -> FuzzerConfig {
        FuzzerConfig {
            ..Default::default()
        }
    }
}

pub fn fuzz(fuzzer_config: &FuzzerConfig) -> Result<(), Box<dyn Error>> {
    let exec = process_exec(fuzzer_config.exec.clone())?;

    let args = match fuzzer_config.arguments.clone() {
        Some(value) => value,
        None => String::from("Unable to read \"exec\" argument."),
    };

    match process_wordlist(fuzzer_config.wordlist.clone()) {
        Ok(value) => fuzz_wordlist(&exec, &value, &args)?,
        Err(_) => {}
    };

    match fuzzer_config.number_range.clone() {
        Some(value) => fuzz_number_range(&exec, value, fuzzer_config.increment, &args)?,
        None => {}
    };

    match fuzzer_config.string_range.clone() {
        Some(value) => fuzz_string_range(&exec, value, fuzzer_config.increment, &args)?,
        None => {}
    };

    Ok(())
}

fn process_exec(exec_option: Option<String>) -> Result<String, Box<dyn Error>> {
    let exec: String = match exec_option {
        Some(value) => value,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unable to parse exec argument."),
            )));
        }
    };

    Ok(exec)
}

fn process_wordlist(wordlist_option: Option<String>) -> Result<String, Box<dyn Error>> {
    let wordlist: String = match wordlist_option {
        Some(value) => value,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(""),
            )));
        }
    };

    Ok(wordlist)
}

fn build_arg_list(args: &str, fuzz_value: &str) -> Vec<String> {
    let mut arg_list: Vec<String> = Vec::new();

    let new_args = args.replace("FUZZ", fuzz_value);

    arg_list = match shellwords::split(&new_args) {
        Ok(value) => value,
        Err(_) => Vec::new(),
    };

    arg_list
}

fn fuzz_wordlist(exec: &str, wordlist: &str, args: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&wordlist);

    if !path.exists() || !path.is_file() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File \"{}\" not found.", wordlist),
        )));
    }

    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let arg_list = build_arg_list(args, &line?);
        run_exec(exec, &arg_list)?;
    }

    Ok(())
}

fn fuzz_number_range(
    exec: &str,
    number_range: (i128, i128),
    increment: usize,
    args: &str,
) -> Result<(), Box<dyn Error>> {
    let mut input: String;

    if number_range.0 - number_range.1 <= 0 {
        for n in (number_range.0..number_range.1).step_by(increment) {
            input = n.to_string();
            let arg_list = build_arg_list(args, &input);
            run_exec(exec, &arg_list)?;
        }
    } else {
        for n in (number_range.1..number_range.0).rev().step_by(increment) {
            input = n.to_string();
            let arg_list = build_arg_list(args, &input);
            run_exec(exec, &arg_list)?;
        }
    }

    Ok(())
}

fn fuzz_string_range(
    exec: &str,
    string_range: (usize, usize),
    increment: usize,
    args: &str,
) -> Result<(), Box<dyn Error>> {
    let mut input: String;

    for n in (string_range.0..string_range.1).step_by(increment) {
        input = "A".repeat(string_range.0 + n);
        let arg_list = build_arg_list(args, &input);
        run_exec(exec, &arg_list)?;
    }

    Ok(())
}

fn run_exec(exec: &str, arg_list: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut output: String;

    println!("Running: {} {:?}", exec, arg_list);
    /*
        match Command::new(exec).args(arg_list).output() {
            Ok(result) => {
                if result.status.success() {
                    // TODO: come up with a better way to indentify the status code
                    // exec_status = ExecStatus::OK;
                }

                output = if result.status.success() {
                    String::from_utf8(result.stdout)?
                } else {
                    String::from_utf8(result.stderr)?
                };
            }
            Err(e) => {
                //exec_status = ExecStatus::ERR;
                output = e.to_string();
            }
        };
    */
    // TODO: handle errors properly, include exit codes and color-coding
    //   println!("{}", output);

    Ok(())
}
